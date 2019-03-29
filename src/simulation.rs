use rand::Rng;
use crate::bug::Bug;
use crate::plant::Plant;
use crate::vector2::Vector2;

pub type Size = (usize, usize);

pub struct Simulation {
    pub tick: u64,
    pub plants: Vec<Plant>,
    pub bugs: Vec<Bug>,
    pub size: Size,
}

fn get_unused_location(size: &Size, locations: &Vec<Vector2>) -> Vector2 {
    let mut rng = rand::thread_rng();
    loop {
        let location = (rng.gen_range(0, size.0) as i32, rng.gen_range(0, size.1) as i32);

        let has_overlapping_point = locations.iter().any(|p| *p == location);

        if !has_overlapping_point {
            return location;
        }
    }
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
            location: *p
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
        .skip(total_count)
        .map(|p| Plant {
            location: *p
        })
        .collect();
}

pub fn create_simulation() -> Simulation {
    let size = (30, 30);
    let bugs = create_first_bugs(&size, 60);
    let plants = create_first_plants(&size, &bugs, 20);

    Simulation {
        tick: 0,
        bugs,
        plants,
        size,
    }
}