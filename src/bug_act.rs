use crate::bug::Bug;
use crate::plant::Plant;
use crate::vector2;
use crate::simulation::Simulation;
use crate::simulation::Size;
use crate::util;

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

fn is_within((x, y): vector2::Vector2, (width, height): Size) -> bool {
    return x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

fn find_free_locations_around(simulation: &Simulation, point: vector2::Vector2) -> Vec<vector2::Vector2> {
    let all_used_points = get_all_used_points(simulation);

    let potential_places: Vec<vector2::Vector2> =
        (-1..2).flat_map(|x: i32|
            (-1..2).map(move |y: i32| (x, y)))
        .map(|diff| vector2::add(diff, point))
        .filter(|&p| is_within(p, simulation.size))
        .collect();

    return potential_places
        .into_iter()
        .filter(|point_around|
            all_used_points
                .iter()
                .all(|used_point| used_point != point_around))
        .collect();
}

fn move_towards(bug: &Bug, plant: Plant, free_locations: Vec<vector2::Vector2>) -> vector2::Vector2 {
    let direction = vector2::to_unit_vector(vector2::subtract(plant.location, bug.location));
    let movement = direction;

    let intended_location = vector2::add(bug.location, movement);

    if free_locations.iter().all(|&l| l != intended_location) {
        return util::pick_one(&free_locations);
    }

    return intended_location;
}

fn find_next_location(simulation: &Simulation, bug: &Bug) -> Option<vector2::Vector2> {
    let free_locations = find_free_locations_around(simulation, bug.location);
    if free_locations.len() == 0 {
        return None
    }

    return Some(match find_closest_plant(&bug, &simulation.plants) {
        Some(plant) => move_towards(&bug, plant, free_locations),
        None => util::pick_one(&free_locations),
    });
}

impl util::Actor for Bug {
    fn act(&mut self, simulation: &mut Simulation) {
        self.location = match find_next_location(simulation, self) {
            Some(location) => location,
            None => self.location,
        };
    }
}