use crate::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Bug {
    pub perception_distance: i32,
    pub location: Vector2,
    pub life: i16,
}
