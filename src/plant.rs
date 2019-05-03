use crate::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Plant {
    pub location: Vector2,
    pub life: i16,
    pub time_since_last_seed: i16,
    pub age: i16,
}

#[derive(Copy, Clone)]
pub struct Seed {
    pub seeded: bool,
    pub location: Vector2,
    pub velocity: Vector2
}
