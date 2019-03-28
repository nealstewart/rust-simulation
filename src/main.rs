extern crate rand;
extern crate ndarray;
use rand::Rng;

type Vector2 = (i32, i32);

struct Plant {
    location: Vector2
}

struct Bug {
    perception_distance: i32,
    location: Vector2
}

struct Simulation {
    tick: u64,
    plants: Vec<Plant>,
    bugs: Vec<Bug>
}

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

fn get_unused_location(locations: &Vec<Vector2>) -> Vector2 {
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
            perception_distance: 3,
            location: *p
        })
        .collect();
}

fn create_first_plants(bugs: &Vec<Bug>, total_count: usize) -> Vec<Plant> {
    let mut count = total_count;
    let mut all_points: Vec<Vector2> = bugs.iter().map(|b| b.location).collect();

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

fn sum_point(point: Vector2) -> i32 {
    return point.0.abs() + point.1.abs();
}

fn subtract(a: Vector2, b: Vector2) -> Vector2 {
    return (a.0 - b.0, a.1 - b.1);
}

fn run_tick(simulation: &mut Simulation) {
    simulation.tick += 1;

    let bugs = &simulation.bugs;
    let plants = &simulation.plants;

    for bug in bugs {
        let nearby_plants: Vec<Plant> = plants.iter()
            .filter(|p| sum_point(subtract(p.location, bug.location)) < bug.perception_distance)
            .clone()
            .collect();
    }
}

fn create_world_string(simulation: Simulation) {
    let mut world = ndarray::Array2::<char>::from_elem((WIDTH, HEIGHT), ' ');

    for bug in simulation.bugs {
        world[[bug.location.0 as usize, bug.location.1 as usize]] = 'B';
    }

    for plant in simulation.plants {
        world[[plant.location.0 as usize, plant.location.1 as usize]] = 'P';
    }

    for row in world.genrows() {
        for ch in row.iter() {
            print!("{}", ch);
        }
        print!("\n");
    }
}

fn main() {
    let mut simulation = create_simulation();
    run_tick(&mut simulation);

    create_world_string(simulation);
}
