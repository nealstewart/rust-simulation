use rand::Rng;
use crate::simulation::Simulation;
use crate::simulation::Size;
use crate::vector2;

pub fn pick_one<T: Copy>(vectors: &Vec<T>) -> T {
    let mut rng = rand::thread_rng();
    let val = rng.gen_range(0, vectors.len());

    return vectors[val];
}

pub fn get_neighbouring_spots(size: Size, point: vector2::Vector2) -> Vec<vector2::Vector2> {
    return
        (-1..2).flat_map(|x: vector2::VectorUnit|
            (-1..2).map(move |y: vector2::VectorUnit| (x, y)))
        .map(|diff| vector2::add(diff, point))
        .filter(|&p| is_within(p, size))
        .collect();
}

fn is_within((x, y): vector2::Vector2, (width, height): Size) -> bool {
    return x >= 0 && x < width as vector2::VectorUnit && y >= 0 && y < height as vector2::VectorUnit
}

pub trait Actor {
    fn act(&mut self, simulation: &mut Simulation);
}