use rand::Rng;
use crate::bug_act;
use crate::bug;
use crate::plant_act;
use crate::plant;
use crate::vector2::Vector2;
use crate::vector2;

pub type Size = (usize, usize);

pub struct Simulation {
    pub tick: u64,
    pub plants: Vec<plant::Plant>,
    pub bugs: Vec<bug::Bug>,
    pub size: Size,
    pub seeds: Vec<plant::Seed>,
    pub eggs: Vec<bug::Egg>,
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

fn create_first_bugs(size: &Size, total_count: usize) -> Vec<bug::Bug> {
    let mut count = total_count;
    let mut locations = Vec::new();

    while count > 0 {
        let location = get_unused_location(size, &locations);
        locations.push(location);
        count -= 1;
    }

    return locations.iter()
        .map(bug_act::create_bug)
        .collect();
}

fn create_first_plants(size: &Size, bugs: &Vec<bug::Bug>, total_count: usize) -> Vec<plant::Plant> {
    let mut count = total_count;
    let mut all_points: Vec<Vector2> = bugs.iter().map(|b| b.location).collect();

    while count > 0 {
        let location = get_unused_location(size, &all_points);

        all_points.push(location);
        count -= 1;
    }

    return all_points.iter()
        .skip(bugs.len())
        .map(plant_act::create_plant)
        .collect();
}

pub fn is_location_in_use(simulation: &Simulation, location: vector2::Vector2) -> bool {
    return
            simulation.plants
                .iter()
                .map(|p| p.location)
                .any(|s| s == location) ||
            simulation.bugs.iter()
                .map(|b| b.location)
                .any(|s| s == location);
}

pub fn is_dead_world(simulation: &Simulation) -> bool {
    return simulation.bugs.len() == 0 && simulation.eggs.len() == 0 && simulation.plants.len() == 0 && simulation.seeds.len() == 0;
}

pub fn create_simulation() -> Simulation {
    let size = (40, 100);
    let bugs = create_first_bugs(&size, 20);
    let plants = create_first_plants(&size, &bugs, 20);

    Simulation {
        tick: 0,
        bugs,
        plants,
        size,
        seeds: Vec::new(),
        eggs: Vec::new()
    }
}