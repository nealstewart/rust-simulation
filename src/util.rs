use rand::Rng;
use crate::simulation::Simulation;

pub fn pick_one<T: Copy>(vectors: &Vec<T>) -> T {
    let mut rng = rand::thread_rng();
    let val = rng.gen_range(0, vectors.len());

    return vectors[val];
}

pub trait Actor {
    fn act(&mut self, simulation: &mut Simulation);
}