use crate::vector2::Vector2;
use crate::vector2;

#[derive(Copy, Clone)]
pub struct Bug {
    pub perception_distance: vector2::VectorUnit,
    pub location: Vector2,
    pub life: i16,
}
