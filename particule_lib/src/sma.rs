use crate::environment::Environment;
use crate::AgentImpl;
use crate::Coord;
use crate::fish::Fish;
use rand::{thread_rng, seq::SliceRandom};
pub struct Sma {
    pub environment: Environment,
}

impl Sma {
    pub fn tick(&mut self) {
        self.environment.update_all();
    }

    pub fn new(width: i32, height: i32) -> Sma {
        Sma {
            environment: Environment::new(width, height),
        }
    }

    pub fn remove_agent(&mut self, agent: AgentImpl) {
        self.environment.remove_agent(agent);
    }

    // Unfortunatly we need to pass a RC to have a shared reference
    pub fn gen_agents(&mut self, fish_number: u32) {

        let size = self.environment.height * self.environment.width;

        let mut vec: Vec<i32> = (0..size).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);

        (0..(fish_number as usize)).for_each(|_| {
            let idx = vec.pop().unwrap();
            let x = idx % self.environment.width;
            let y = (idx - x) / self.environment.width;
            let coordinate = Coord(x, y);

            let fish = Fish {
                coordinate,
                previous_coordinate: coordinate,
                breed_count_down: 10, //FOXME
            };

            self.environment.add_agent(Box::new(fish));
        });

    }
}
