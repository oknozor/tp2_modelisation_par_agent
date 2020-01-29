use crate::AgentImpl;
use crate::AgentKind;
use crate::Coord;

use std::collections::HashMap;

pub struct Environment {
    pub board: Vec<Cell>,
    pub agents: HashMap<Coord, AgentImpl>,
    pub width: i32,
    pub height: i32,
}

impl Environment {
    pub fn new(width: i32, height: i32) -> Environment {
        let size = width * height;
        let mut board = vec![];

        for _ in 0..size {
            board.push(Cell::Empty)
        }

        let agents = HashMap::new();

        Environment {
            width,
            height,
            board,
            agents,
        }
    }

    pub fn update_all(&mut self) {
        let updates = self.decide_all();

        updates.iter().for_each(|(from, to)| {
            let mut agent = self.agents.remove(from).expect("Expected an agent");
            agent.update(to);
            self.agents.insert(*to, agent);
            self.swap_cells(from, to);
        });
    }

    fn decide_all(&mut self) -> Vec<(Coord, Coord)> {
        self.agents
            .iter()
            .map(|(_, agent)| {
                let neighbors = self.get_empty_neighbors(agent.coordinate());
                agent.decide(neighbors)
            })
            .filter(|update| update.is_some())
            .map(|update| update.expect("Expected (Coord, Coord), found None"))
            .collect()
    }

    // pub fn breed(&mut self, agent: AgentImpl) {
    //     self.swap_cells(agent.coordinate(), agent.previous_coordinate());

    //     if agent.breed_count_down() == 0 {
    //         self.add_agent(Box::new(Fish {
    //             coordinate: agent.previous_coordinate(),
    //             previous_coordinate: agent.previous_coordinate(),
    //             breed_count_down: 10, //FIXME
    //         }));
    //     }
    // }

    pub fn add_agent(&mut self, agent: AgentImpl) {
        self.set_cell(&agent);
        self.agents.insert(agent.coordinate(), agent);
    }

    pub fn remove_agent(&mut self, agent: AgentImpl) {
        let key = &agent.coordinate();
        let idx = self.get_index(key);
        self.board[idx] = Cell::Empty;
        self.agents.remove(key);
    }

    pub fn swap_cells(&mut self, a: &Coord, b: &Coord) {
        if a != b {
            let idx_a = self.get_index(&a);
            let idx_b = self.get_index(&b);
            self.board.swap(idx_a, idx_b);
        }
    }

    pub fn get_empty_neighbors(&self, coord: Coord) -> Vec<Coord> {
        self.get_neighbor(&coord)
            .iter()
            .cloned()
            .filter(|(cell, _)| cell.is_empty())
            .map(|(_, coord)| coord.clone())
            .collect()
    }

    pub fn get_neighbor(&self, coord: &Coord) -> Vec<(Cell, Coord)> {
        let north = &Coord(coord.0, coord.1 + 1);
        let south = &Coord(coord.0, coord.1 - 1);
        let east = &Coord(coord.0 - 1, coord.1);
        let west = &Coord(coord.0 + 1, coord.1);
        let north_east = &Coord(coord.0 - 1, coord.1 + 1);
        let north_west = &Coord(coord.0 + 1, coord.1 + 1);
        let south_east = &Coord(coord.0 - 1, coord.1 - 1);
        let south_west = &Coord(coord.0 + 1, coord.1 - 1);

        vec![
            (self.get_cell(north), north),
            (self.get_cell(south), south),
            (self.get_cell(east), east),
            (self.get_cell(west), west),
            (self.get_cell(north_west), north_west),
            (self.get_cell(north_east), north_east),
            (self.get_cell(south_east), south_east),
            (self.get_cell(south_west), south_west),
        ]
        .iter()
        .filter(|(cell, _)| cell.is_some())
        .map(|(cell, coord)| (*cell.expect("expected Cell found None"), **coord))
        .collect()
    }

    fn get_cell(&self, coord: &Coord) -> Option<&Cell> {
        if coord.0 < 0 || coord.0 >= self.width || coord.1 < 0 || coord.1 >= self.height {
            None
        } else {
            let idx = self.get_index(coord);
            Some(&self.board[idx])
        }
    }

    fn get_index(&self, coord: &Coord) -> usize {
        (coord.1 * self.width + coord.0) as usize
    }

    fn set_cell(&mut self, agent: &AgentImpl) {
        let idx = self.get_index(&agent.coordinate());
        self.board[idx] = Cell::Filled(agent.get_kind())
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Filled(AgentKind),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        if self.is_empty() {
            "[empty]".into()
        } else {
            "[agent]".into()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::environment::Environment;
    use crate::fish::Fish;
    use crate::Coord;

    #[test]
    fn should_set_cell() {
        let mut env = Environment::new(5, 5);
        let agent = Fish {
            breed_count_down: 1,
            coordinate: Coord(0, 0),
            previous_coordinate: Coord(0, 0),
        };

        env.add_agent(Box::new(agent));

        let expected = env.agents.get(&Coord(0, 0)).map(|agent| agent.coordinate());

        assert_eq!(env.get_index(&Coord(0, 0)), 0);
        assert_eq!(expected, Some(Coord(0, 0)));
    }

    #[test]
    fn should_update() {
        let mut env = Environment::new(5, 5);
        let agent = Fish {
            breed_count_down: 1,
            coordinate: Coord(0, 0),
            previous_coordinate: Coord(0, 0),
        };

        env.add_agent(Box::new(agent));

        let expected = env.agents.get(&Coord(0, 0)).map(|agent| agent.coordinate());

        assert_eq!(env.get_index(&Coord(0, 0)), 0);
        assert_eq!(expected, Some(Coord(0, 0)));

        env.update_all();

        let expected = env.agents.get(&Coord(0, 0)).map(|agent| agent.coordinate());
        assert_eq!(expected, None);

    }
}
