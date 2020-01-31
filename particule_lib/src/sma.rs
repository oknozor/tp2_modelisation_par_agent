use crate::environment::Environment;
use crate::wator::fish::Fish;
use crate::wator::shark::Shark;
use crate::Coord;
use crate::BORDERLESS;
use crate::FISH_BREED_COUNT_DOWN;
use crate::MAX_HEIGTH;
use crate::MAX_WIDTH;
use crate::SHARK_BREED_COUNT_DOWN;
use crate::SHARK_STARVE_COUNT_DOWN;

use rand::{seq::SliceRandom, thread_rng, Rng};
pub struct Sma {
    pub environment: Environment,
}

impl Sma {
    pub fn tick(&mut self) {
        self.environment.update_all();
    }

    pub fn new(
        width: i32,
        height: i32,
        fish_breed_time: i32,
        shark_breed_time: i32,
        shark_starve_time: i32,
        borderless: bool,
    ) -> Sma {
        unsafe {
            FISH_BREED_COUNT_DOWN = fish_breed_time;
            SHARK_BREED_COUNT_DOWN = shark_breed_time;
            SHARK_STARVE_COUNT_DOWN = shark_starve_time;
            MAX_HEIGTH = height;
            MAX_WIDTH = width;
            BORDERLESS = borderless;
        }

        Sma {
            environment: Environment::new(width, height),
        }
    }

    // Unfortunatly we need to pass a RC to have a shared reference
    pub fn gen_agents(&mut self, fish_number: u32, shark_number: u32) {
        let size = self.environment.height * self.environment.width;

        let mut vec: Vec<i32> = (0..size).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);

        (0..(fish_number as usize)).for_each(|_| {
            let idx = vec.pop().unwrap();
            let x = idx % self.environment.width;
            let y = (idx - x) / self.environment.width;
            let coordinate = Coord(x, y);

            let fish = Fish::new(coordinate);
            self.environment.add_agent(Box::new(fish));
        });

        (0..(shark_number as usize)).for_each(|_| {
            let idx = vec.pop().unwrap();
            let x = idx % self.environment.width;
            let y = (idx - x) / self.environment.width;
            let coordinate = Coord(x, y);

            let shark = Shark::new(coordinate);
            self.environment.add_agent(Box::new(shark));
        });
    }

    pub fn gen_agents_grouped(&mut self, fish_number: u32, shark_number: u32) {
        let size = self.environment.height * self.environment.width;
        let idx = thread_rng().gen_range(0, size);

        let target = Coord::from_idx(idx);
        let fish = Fish::new(target);
        self.environment.add_agent(Box::new(fish));
        (0..((fish_number - 1) as usize)).for_each(|_| {
            let available = self.environment.get_adjacent();
            let neigh_idx = thread_rng().gen_range(0, available.len());
            let target: Coord = available[neigh_idx];
            let fish = Fish::new(target);
            self.environment.add_agent(Box::new(fish));
        });

        (0..(shark_number as usize)).for_each(|_| {
            let available = self.environment.get_adjacent();
            let neigh_idx = thread_rng().gen_range(0, available.len());
            let target: Coord = available[neigh_idx];
            let shark = Shark::new(target);
            self.environment.add_agent(Box::new(shark));
        });
    }
}
