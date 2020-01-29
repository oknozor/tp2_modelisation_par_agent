use crate::Agent;
use crate::AgentKind;
use crate::Coord;
use rand::{thread_rng, Rng};
pub struct Fish {
    pub coordinate: Coord,
    pub previous_coordinate: Coord,
    pub breed_count_down: u8,
}

impl Agent for Fish {
    fn update(&mut self, new_position: &Coord) {
        self.breed_count_down -= 1;
        self.previous_coordinate = self.coordinate;
        self.coordinate = *new_position;
    }

    fn decide(&self, neighbors: Vec<Coord>) -> Option<(Coord, Coord)> {
        if neighbors.is_empty() {
            None
        } else {
            let from = self.coordinate;
            let mut rng = thread_rng();
            let idx = rng.gen_range(0, neighbors.len());
            let to = neighbors[idx];
            
            Some((from, to))
        }
    }

    fn get_kind(&self) -> crate::AgentKind {
        AgentKind::Fish
    }
    fn coordinate(&self) -> Coord {
        self.coordinate
    }
    fn previous_coordinate(&self) -> Coord {
        self.previous_coordinate
    }
    fn breed_count_down(&self) -> u8 {
        self.breed_count_down
    }

    fn set_breed_count_down(&mut self, value: u8) {
        self.breed_count_down = value
    }
}
