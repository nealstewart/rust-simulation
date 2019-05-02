use crate::vector2::Vector2;
use crate::vector2;

#[derive(Copy, Clone)]
pub struct Bug {
    pub perception_distance: vector2::VectorUnit,
    pub location: Vector2,
    pub life: i16,
    pub time_until_lay_egg: i16
}

pub struct Egg {
    pub time_until_hatch: i16
}
