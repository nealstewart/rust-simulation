use crate::vector2::Vector2;
use crate::simulation::Simulation;
use crate::util::Actor;

#[derive(Copy, Clone)]
pub struct Plant {
    pub location: Vector2
}

impl Actor for Plant {
    fn act(&mut self, simulation: &mut Simulation) {

    }
}




