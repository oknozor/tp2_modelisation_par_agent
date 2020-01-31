use crate::get_fish_breed_time;
use crate::Agent;
use crate::AgentImpl;
use crate::AgentKind;
use crate::Cell;
use crate::Coord;
use crate::Decision;
use std::convert::TryInto;

use rand::{thread_rng, Rng};
#[derive(Clone, Debug)]
pub struct Fish {
    pub coordinate: Coord,
    pub breed_count_down: i32,
}

impl Fish {
    pub fn new(coordinate: Coord) -> Fish {
        Fish {
            coordinate,
            breed_count_down: get_fish_breed_time(),
        }
    }
}

impl Agent for Fish {
    fn update(&mut self) {
        self.breed_count_down -= 1;
    }

    fn decide(&self, neighbors: &Vec<Cell>) -> Decision {
        if neighbors.is_empty() {
            Decision::Stall(self.coordinate)
        } else {
            let from = self.coordinate;
            let mut rng = thread_rng();
            let idx = rng.gen_range(0, neighbors.len());
            let destination = neighbors[idx].clone().try_into();
            if let Ok(to) = destination {
                if self.breed_count_down == 0 {
                    Decision::MoveAndBreed(from, to)
                } else {
                    Decision::Move(from, to)
                }
            } else {
                unreachable!()
            }
        }
    }

    fn get_kind(&self) -> crate::AgentKind {
        AgentKind::Fish
    }
    fn coordinate(&self) -> Coord {
        self.coordinate
    }

    fn breed(&mut self) -> AgentImpl {
        self.breed_count_down = get_fish_breed_time();
        Box::new(Fish {
            coordinate: self.coordinate,
            breed_count_down: get_fish_breed_time(),
        })
    }
    fn get_color(&self) -> (f32, f32, f32) {
        (0.0, 1.0, 0.0)
    }
    fn set_coordinate(&mut self, coord: Coord) {
        self.coordinate = coord
    }
    fn clone_boxed(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }

    fn reset_starve_count_down(&mut self) {
        unreachable!("Fish don't starve")
    }
}
