use crate::vector2::Vector2;
use crate::vector2;

#[derive(Copy, Clone)]
pub struct Bug {
    pub perception_distance: vector2::VectorUnit,
    pub location: Vector2,
    pub life: i16,
    pub time_since_last_egg: i16,
    pub age: i16
}

#[derive(Copy, Clone)]
pub struct Egg {
    pub time_until_hatch: i16,
    pub location: Vector2,
}
