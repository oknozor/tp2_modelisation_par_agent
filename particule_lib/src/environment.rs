use crate::borderless;
use crate::trace::TickTrace;
use crate::AgentImpl;
use crate::AgentKind;
use crate::Cell;
use crate::Coord;
use crate::Decision;
use std::convert::TryInto;

pub struct Environment {
    pub board: Vec<Cell>,
    pub width: i32,
    pub height: i32,
}

impl Environment {
    pub fn new(width: i32, height: i32) -> Environment {
        let size = width * height;
        let board = vec![];

        let mut env = Environment {
            width,
            height,
            board,
        };

        let mut board = vec![];

        for idx in 0..size {
            board.push(Cell::Empty(Coord::from_idx(idx)))
        }

        env.board = board;

        env
    }

    pub fn update_all(&mut self) {
        let size = self.board.len();

        let mut trace = TickTrace::new();
        for idx in 0..size {
            let decision = if let Cell::Filled(agent) = &self.board[idx] {
                match agent.get_kind() {
                    AgentKind::Fish => {
                        let neighbors = &self.get_empty_neighbors(agent.coordinate());
                        Some((agent.decide(neighbors), AgentKind::Fish))
                    }
                    AgentKind::Shark => {
                        let neighbors = &self.get_shark_neighbors(agent.coordinate());
                        Some((agent.decide(neighbors), AgentKind::Shark))
                    }
                }
            } else {
                None
            };

            if let Some((decision, agent_kind)) = decision {
                match decision {
                    Decision::Stall(position) => self.update_agent(position),
                    Decision::Move(from, to) => {
                        self.update_agent(from);
                        &self.move_agent(from, to);
                    }
                    Decision::MoveAndBreed(from, to) => {
                        self.update_agent(from);
                        &self.breed_and_move_agent(from, to);

                        trace.birth(agent_kind);
                    }
                    Decision::EatAndMove(from, to) => {
                        self.update_agent_and_reset_starve(from);
                        self.remove_agent(to);
                        self.move_agent(from, to);
                        trace.death(AgentKind::Fish);
                    }
                    Decision::EatAndBreed(from, to) => {
                        self.update_agent_and_reset_starve(from);
                        self.remove_agent(to);
                        self.breed_and_move_agent(from, to);
                        trace.death(AgentKind::Fish);
                        trace.birth(AgentKind::Shark);
                    }
                    Decision::Starve(position) => {
                        self.remove_agent(position);
                        trace.death(AgentKind::Shark);
                    }
                };
            }
        }
        println!("{}", trace);
    }

    pub fn update_agent(&mut self, coord: Coord) {
        let agent: &mut AgentImpl = self.get_mut_agent(coord).unwrap();
        agent.update();
    }

    pub fn update_agent_and_reset_starve(&mut self, coord: Coord) {
        let agent: &mut AgentImpl = self.get_mut_agent(coord).unwrap();
        agent.update();
        agent.reset_starve_count_down();
    }

    pub fn add_agent(&mut self, agent: AgentImpl) {
        let coord = &agent.coordinate();
        let idx = coord.as_idx();
        if self.board[idx].is_empty() {
            self.board[idx] = Cell::Filled(agent)
        }
    }

    pub fn remove_agent(&mut self, coord: Coord) {
        let idx = coord.as_idx();
        self.board[idx] = Cell::Empty(coord);
    }

    pub fn get_empty_neighbors(&self, coord: Coord) -> Vec<Cell> {
        self.get_neighbor(coord)
            .iter()
            .cloned()
            .filter(|cell| cell.is_empty())
            .collect()
    }

    pub fn get_shark_neighbors(&self, coord: Coord) -> Vec<Cell> {
        self.get_neighbor(coord)
            .iter()
            .cloned()
            .filter(|cell| cell.is_fish() || cell.is_empty())
            .collect()
    }

    fn get_neighbor(&self, coord: Coord) -> Vec<Cell> {
        let north;
        let south;
        let east;
        let west;
        let north_east;
        let north_west;
        let south_east;
        let south_west;

        if !borderless() {
            north = coord + Coord(0, 1);
            south = coord + Coord(0, -1);
            east = coord + Coord(-1, 0);
            west = coord + Coord(1, 0);
            north_east = coord + Coord(-1, 1);
            north_west = coord + Coord(1, 1);
            south_east = coord + Coord(-1, -1);
            south_west = coord + Coord(1, -1);
        } else {
            // It is not a multiplication !
            // Mul operator overloading is used to get toric position
            north = coord * Coord(0, 1);
            south = coord * Coord(0, -1);
            east = coord * Coord(-1, 0);
            west = coord * Coord(1, 0);
            north_east = coord * Coord(-1, 1);
            north_west = coord * Coord(1, 1);
            south_east = coord * Coord(-1, -1);
            south_west = coord * Coord(1, -1);
        }

        vec![
            self.get_cell(north),
            self.get_cell(south),
            self.get_cell(east),
            self.get_cell(west),
            self.get_cell(north_west),
            self.get_cell(north_east),
            self.get_cell(south_east),
            self.get_cell(south_west),
        ]
    }

    fn move_agent(&mut self, from: Coord, to: Coord) {
        let agent: &mut Result<AgentImpl, String> = &mut self.get_cell(from).try_into();
        if let Ok(agent) = agent {
            self.set_empty_cell(agent.coordinate());
            agent.set_coordinate(to);
            self.set_agent_cell(&agent)
        }
    }

    fn breed_and_move_agent(&mut self, from: Coord, to: Coord) {
        let agent: &mut Result<AgentImpl, String> = &mut self.get_cell(from).try_into();

        if let Ok(agent) = agent {
            self.set_agent_cell(&agent.breed());
            agent.set_coordinate(to);
            self.set_agent_cell(&agent);
        }
    }

    pub fn is_shark_at(&self, coord: Coord) -> bool {
        let cell = self.get_cell(coord);

        if let Cell::Filled(agent) = cell {
            if agent.get_kind() == AgentKind::Shark {
                return true;
            }
        }

        return false;
    }

    fn get_cell(&self, coord: Coord) -> Cell {
        self.board[coord.as_idx()].clone()
    }

    fn get_mut_agent(&mut self, coord: Coord) -> Option<&mut AgentImpl> {
        if let Cell::Filled(agent) = &mut self.board[coord.as_idx()] {
            Some(agent)
        } else {
            None
        }
    }

    fn set_agent_cell(&mut self, agent: &AgentImpl) {
        self.board[agent.coordinate().as_idx()] = Cell::Filled(agent.clone());
    }

    fn set_empty_cell(&mut self, coord: Coord) {
        self.board[coord.as_idx()] = Cell::Empty(coord);
    }

    pub fn get_adjacent(&self) -> Vec<Coord> {
        let cells: Vec<Vec<Cell>> = self
            .board
            .iter()
            .filter(|cell| !cell.is_empty())
            .map(|cell| self.get_empty_neighbors(cell.to_coord_unchecked()))
            .collect();

        let coords = cells
            .iter()
            .flatten()
            .map(|cell| cell.try_into())
            .map(|coord| coord.unwrap())
            .collect::<Vec<Coord>>();

        coords
    }
}
