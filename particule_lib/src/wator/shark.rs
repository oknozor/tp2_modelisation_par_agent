use crate::get_shark_breed_time;
use crate::get_shark_starve_time;
use crate::Agent;
use crate::AgentImpl;
use crate::AgentKind;
use crate::Cell;
use crate::Coord;
use crate::Decision;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Shark {
    pub coordinate: Coord,
    pub breed_count_down: i32,
    pub starve_count_down: i32,
}

impl Shark {
    pub fn new(coordinate: Coord) -> Shark {
        Shark {
            coordinate,
            breed_count_down: get_shark_breed_time(),
            starve_count_down: get_shark_starve_time(),
        }
    }
}

impl Agent for Shark {
    fn update(&mut self) {
        self.starve_count_down -= 1;
        self.breed_count_down -= 1;
    }

    fn decide(&self, neighbors: &Vec<Cell>) -> Decision {
        if self.starve_count_down < 0 {
            return Decision::Starve(self.coordinate);
        }

        let from = self.coordinate;
        let mut rng = thread_rng();

        let fish_in_neighbor = neighbors
            .iter()
            .filter(|cell| !cell.is_empty())
            .collect::<Vec<&Cell>>();

        let decision = if !fish_in_neighbor.is_empty() {
            let idx = rng.gen_range(0, fish_in_neighbor.len());
            let to: Result<AgentImpl, String> = neighbors[idx].clone().try_into();

            if let Ok(to) = to {
                if self.breed_count_down < 0 {
                    Decision::EatAndBreed(from, to.coordinate())
                } else {
                    Decision::EatAndMove(from, to.coordinate())
                }
            } else {
                Decision::Stall(self.coordinate)
            }
        } else if !neighbors.is_empty() {
            let idx = rng.gen_range(0, neighbors.len());
            let to = neighbors[idx].clone().try_into();

            if let Ok(to) = to {
                if self.breed_count_down < 0 {
                    Decision::MoveAndBreed(from, to)
                } else {
                    Decision::Move(from, to)
                }
            } else {
                Decision::Stall(self.coordinate)
            }
        } else {
            Decision::Stall(self.coordinate)
        };

        decision
    }

    fn get_kind(&self) -> crate::AgentKind {
        AgentKind::Shark
    }
    fn coordinate(&self) -> Coord {
        self.coordinate
    }

    fn breed(&mut self) -> AgentImpl {
        self.breed_count_down = get_shark_breed_time();
        Box::new(Shark {
            coordinate: self.coordinate,
            breed_count_down: get_shark_breed_time(),
            starve_count_down: get_shark_starve_time(),
        })
    }
    fn get_color(&self) -> (f32, f32, f32) {
        (1.0, 0.0, 0.0)
    }
    fn set_coordinate(&mut self, coord: Coord) {
        self.coordinate = coord
    }
    fn clone_boxed(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }

    fn reset_starve_count_down(&mut self) {
        self.starve_count_down = get_shark_starve_time()
    }
}
