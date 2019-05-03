
use rand::Rng;
use crate::simulation::{is_location_in_use, Simulation};
use crate::simulation::Size;
use crate::util::Actor;
use crate::util;
use crate::vector2;
use crate::vector2::Vector2;
use crate::plant::Plant;
use crate::plant::Seed;

const MINIMUM_TIME_SINCE_LAST_SEED: i16 = 40;
const MAXIMUM_AGE_OF_PLANT: i16 = 100;

fn should_seed() -> bool {
    return rand::thread_rng().gen_range(0, 100) < 10;
}

fn should_die() -> bool {
    return rand::thread_rng().gen_range(0, 100) < 60;
}

pub fn is_dead(plant: &Plant) -> bool {
    return plant.age > MAXIMUM_AGE_OF_PLANT && should_die();
}

impl Actor for Plant {
    fn act(&mut self, simulation: &mut Simulation) {
        self.time_since_last_seed += 1;
        self.age += 1;

        if self.time_since_last_seed < MINIMUM_TIME_SINCE_LAST_SEED || !should_seed() {
            return
        }

        if is_dead(self) {
            self.life = 0;
        }

        let neighbouring_spaces: Vec<vector2::Vector2> = util::get_neighbouring_spots(simulation.size, self.location)
            .iter()
            .filter(|s| !is_location_in_use(simulation, **s))
            .cloned()
            .collect();

        let mut seeds = (0..neighbouring_spaces.len()).map(|i| Seed {
            seeded: false,
            location: neighbouring_spaces[i],
            velocity: vector2::times(vector2::subtract(neighbouring_spaces[i], self.location), rand::thread_rng().gen_range(1, 20))
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
        age: 0
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

        self.velocity = vector2::divide(self.velocity, 2);
        self.location = constrain(vector2::add(self.location, self.velocity), simulation.size);

        if self.velocity.0.abs() >= 1 || self.velocity.1.abs() >= 1 {
            return
        }

        self.velocity = (0, 0);

        if is_location_in_use(simulation, self.location) {
            return
        }

        self.seeded = true;
        simulation.plants.push(Plant {
            location: self.location,
            life: 4,
            time_since_last_seed: 0,
            age: 0
        })
    }
}
