
use rand::Rng;
use crate::simulation::Simulation;
use crate::simulation;
use crate::simulation::Size;
use crate::util::Actor;
use crate::util;
use crate::vector2;
use crate::vector2::Vector2;
use crate::plant::Plant;
use crate::plant::Seed;

const MINIMUM_TIME_SINCE_LAST_SEED: i16 = 40;

fn should_seed() -> bool {
    return rand::thread_rng().gen_range(0, 100) < 10;
}

impl Actor for Plant {
    fn act(&mut self, simulation: &mut Simulation) {
        self.time_since_last_seed += 1;
        if self.time_since_last_seed < MINIMUM_TIME_SINCE_LAST_SEED || !should_seed() {
            return
        }

        let neighbouring_spaces = util::get_neighbouring_spots(simulation.size, self.location);
        let mut seeds = (0..neighbouring_spaces.len()).map(|i| Seed {
            seeded: false,
            location: neighbouring_spaces[i],
            velocity: vector2::times(vector2::subtract(neighbouring_spaces[i], self.location), rand::thread_rng().gen_range(1, 10))
        }).collect();

        simulation.seeds.append(&mut seeds);
        self.time_since_last_seed = 0;
    }
}

pub fn create_plant(location: &vector2::Vector2) -> Plant {
    Plant {
        location: *location,
        life: 10,
        time_since_last_seed: 0,
    }
}

fn minmax<T: PartialOrd>(val: T, min: T, max: T) -> T {
    return if val > max {
        max
    } else if val < min {
        min
    } else {
        val
    };

}

fn constrain(vector: Vector2, size: Size) -> Vector2 {
    let x = minmax(vector.0, 0, size.0 as i32 - 1);
    let y = minmax(vector.1, 0, size.1 as i32 - 1);
    return (x, y);
}

impl Actor for Seed{
    fn act(&mut self, simulation: &mut Simulation) {
        if self.seeded {
            return
        }

        let used_spots = simulation::get_all_used_points(simulation);
        self.velocity = vector2::divide(self.velocity, 2);
        self.location = constrain(vector2::add(self.location, self.velocity), simulation.size);

        if self.velocity.0.abs() >= 1 || self.velocity.1.abs() >= 1 {
            return
        }

        self.velocity = (0, 0);

        if used_spots.iter().any(|s| *s == self.location) {
            return
        }

        self.seeded = true;
        simulation.plants.push(Plant {
            location: self.location,
            life: 4,
            time_since_last_seed: 0
        })
    }
}
