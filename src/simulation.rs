use rand::Rng;
use crate::bug::Bug;
use crate::plant::Plant;
use crate::plant;
use crate::vector2::Vector2;
use crate::vector2;

pub type Size = (usize, usize);

pub struct Simulation {
    pub tick: u64,
    pub plants: Vec<Plant>,
    pub bugs: Vec<Bug>,
    pub size: Size,
    pub seeds: Vec<plant::Seed>,
}

fn get_unused_location(size: &Size, locations: &Vec<Vector2>) -> Vector2 {
    let mut rng = rand::thread_rng();
    loop {
        let location = (rng.gen_range(0, size.0) as vector2::VectorUnit, rng.gen_range(0, size.1) as vector2::VectorUnit);

        let has_overlapping_point = locations.iter().any(|p| *p == location);

        if !has_overlapping_point {
            return location;
        }
    }
}

pub fn get_all_used_points(simulation: &Simulation) -> Vec<vector2::Vector2> {
    return simulation.plants.iter().map(|p| p.location)
        .chain(simulation.bugs.iter().map(|b| b.location))
        .collect();
}

fn create_first_bugs(size: &Size, total_count: usize) -> Vec<Bug> {
    let mut count = total_count;
    let mut locations = Vec::new();

    while count > 0 {
        let location = get_unused_location(size, &locations);
        locations.push(location);
        count -= 1;
    }

    return locations.iter()
        .map(|p| Bug {
            perception_distance: 10,
            location: *p,
            life: 20
        })
        .collect();
}

fn create_first_plants(size: &Size, bugs: &Vec<Bug>, total_count: usize) -> Vec<Plant> {
    let mut count = total_count;
    let mut all_points: Vec<Vector2> = bugs.iter().map(|b| b.location).collect();

    while count > 0 {
        let location = get_unused_location(size, &all_points);

        all_points.push(location);
        count -= 1;
    }

    return all_points.iter()
        .skip(bugs.len())
        .map(|p| Plant {
            location: *p,
            life: 10,
            time_since_last_seed: 0,
        })
        .collect();
}

pub fn create_simulation() -> Simulation {
    let size = (30, 60);
    let bugs = create_first_bugs(&size, 20);
    let plants = create_first_plants(&size, &bugs, 20);

    Simulation {
        tick: 0,
        bugs,
        plants,
        size,
        seeds: Vec::new()
    }
}