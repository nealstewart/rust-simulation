extern crate rand;
extern crate ndarray;
use std::{thread, time};
use rand::Rng;
mod vector2;

#[derive(Copy, Clone)]
struct Plant {
    location: vector2::Vector2
}

#[derive(Copy, Clone)]
struct Bug {
    perception_distance: i32,
    location: vector2::Vector2
}

struct Simulation {
    tick: u64,
    plants: Vec<Plant>,
    bugs: Vec<Bug>
}

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

fn get_unused_location(locations: &Vec<vector2::Vector2>) -> vector2::Vector2 {
    let mut rng = rand::thread_rng();
    loop {
        let location = (rng.gen_range(0, WIDTH) as i32, rng.gen_range(0, HEIGHT) as i32);

        let has_overlapping_point = locations.iter().any(|p| *p == location);

        if !has_overlapping_point {
            return location;
        }
    }
}

fn create_first_bugs(total_count: usize) -> Vec<Bug> {
    let mut count = total_count;
    let mut locations = Vec::new();

    while count > 0 {
        let location = get_unused_location(&locations);
        locations.push(location);
        count -= 1;
    }

    return locations.iter()
        .map(|p| Bug {
            perception_distance: 2,
            location: *p
        })
        .collect();
}

fn create_first_plants(bugs: &Vec<Bug>, total_count: usize) -> Vec<Plant> {
    let mut count = total_count;
    let mut all_points: Vec<vector2::Vector2> = bugs.iter().map(|b| b.location).collect();

    while count > 0 {
        let location = get_unused_location(&all_points);

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

fn create_simulation() -> Simulation {
    let bugs = create_first_bugs(20);
    let plants = create_first_plants(&bugs, 20);

    Simulation {
        tick: 0,
        bugs: bugs,
        plants: plants,
    }
}

fn find_closest_plant(bug: &Bug, plants: &Vec<Plant>) -> Option<Plant> {
    let mut distances: Vec<(Plant, i32)> = plants
        .into_iter()
        .cloned()
        .map(|p| (p, vector2::sum_point(vector2::subtract(p.location, bug.location))))
        .filter(|i| i.1 < bug.perception_distance)
        .collect();

    if distances.len() == 0 {
        return None;
    }

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    return Some(distances[0].0);
}

fn get_all_used_points(simulation: &Simulation) -> Vec<vector2::Vector2> {
    return simulation.plants.iter().map(|p| p.location)
        .chain(simulation.bugs.iter().map(|b| b.location))
        .collect();
}

fn find_free_locations_around(simulation: &Simulation, point: vector2::Vector2) -> Vec<vector2::Vector2> {
    let all_used_points = get_all_used_points(simulation);

    let potential_places: Vec<vector2::Vector2> =
        (-1..2).flat_map(|x: i32|
            (-1..2).map(move |y: i32| (x, y)))
        .map(|diff| vector2::add(diff, point))
        .filter(|&(x, y)| x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32)
        .collect();

    return potential_places
        .into_iter()
        .filter(|point_around|
            all_used_points
                .iter()
                .all(|used_point| used_point != point_around))
        .collect();
}

fn move_towards(bug: &Bug, plant: Plant, free_locations: &Vec<vector2::Vector2>) -> vector2::Vector2 {
    let direction = vector2::to_unit_vector(vector2::subtract(plant.location, bug.location));
    let movement = direction;

    let intended_location = vector2::add(bug.location, movement);

    if free_locations.iter().all(|&l| l != intended_location) {
        return pick_one(free_locations);
    }

    return intended_location;
}

fn pick_one(vectors: &Vec<vector2::Vector2>) -> vector2::Vector2 {
    let mut rng = rand::thread_rng();

    return vectors[rng.gen_range(0, vectors.len())];
}

fn find_next_location(simulation: &Simulation, bug: &Bug) -> Option<vector2::Vector2> {
    let free_locations = find_free_locations_around(simulation, bug.location);
    if free_locations.len() == 0 {
        return None
    }

    return Some(match find_closest_plant(&bug, &simulation.plants) {
        Some(plant) => move_towards(&bug, plant, &free_locations),
        None => pick_one(&free_locations),
    })
}

fn run_tick(simulation: &mut Simulation) {
    simulation.tick += 1;

    let locations: Vec<vector2::Vector2> = simulation.bugs.iter()
        .map(|bug|
            match find_next_location(&simulation, bug) {
                Some(location) => location,
                None => bug.location
            })
        .collect();

    for (i, location) in locations.iter().enumerate() {
        simulation.bugs[i].location = *location;
    }
}

fn create_world_string(simulation: &Simulation) {
    let mut world = ndarray::Array2::<char>::from_elem((WIDTH, HEIGHT), ' ');

    for bug in &simulation.bugs {
        world[[bug.location.0 as usize, bug.location.1 as usize]] = '*';
    }

    for plant in &simulation.plants {
        world[[plant.location.0 as usize, plant.location.1 as usize]] = 'P';
    }

    for row in world.genrows() {
        for ch in row.iter() {
            print!("{}", ch);
        }
        print!("\n");
    }
    println!("--------------")
}

fn main() {
    let mut simulation = create_simulation();

    loop {
        run_tick(&mut simulation);
        create_world_string(&simulation);
        thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
