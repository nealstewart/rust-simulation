use crate::bug::Bug;
use crate::plant::Plant;
use crate::vector2;
use crate::simulation::Simulation;
use crate::util;
use crate::simulation;

impl util::Actor for Bug {
    fn act(&mut self, simulation: &mut Simulation) {
        let neighbouring_spots = util::get_neighbouring_spots(simulation.size, self.location);

        let mut nearby_plants: Vec<&mut Plant> =
            simulation.plants.iter_mut()
                .filter(|p| neighbouring_spots.iter().any(|l| *l == p.location))
                .collect();

        if nearby_plants.len() > 0 {
            let first_plant = &mut nearby_plants[0];
            first_plant.life -= 1;
            self.life += 2;
            return;
        }

        let old_location = self.location;
        let new_location = match find_next_location(simulation, self) {
            Some(location) => location,
            None => self.location,
        };
        self.location = new_location;

        if old_location != new_location {
            self.life -= 1;
        }

    }
}

fn find_closest_plant(bug: &Bug, plants: &Vec<Plant>) -> Option<Plant> {
    let mut distances: Vec<(Plant, vector2::VectorUnit)> = plants
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

fn find_free_locations_around(simulation: &Simulation, point: vector2::Vector2) -> Vec<vector2::Vector2> {
    let all_used_points = simulation::get_all_used_points(simulation);

    let potential_places: Vec<vector2::Vector2> = util::get_neighbouring_spots(simulation.size, point);

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

