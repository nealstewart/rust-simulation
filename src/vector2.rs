pub type Vector2 = (i32, i32);

pub fn sum_point(point: Vector2) -> i32 {
    return point.0.abs() + point.1.abs();
}

pub fn subtract(a: Vector2, b: Vector2) -> Vector2 {
    return (a.0 - b.0, a.1 - b.1);
}

pub fn add(a: Vector2, b: Vector2) -> Vector2 {
    return (a.0 + b.0, a.1 + b.1);
}

pub fn to_unit_vector(a: Vector2) -> Vector2 {
    let mut greater = if a.0.abs() > a.1.abs() {
        a.0.abs()
    } else {
        a.1.abs()
    };
    if greater == 0 {
        greater = 1;
    }
    return (a.0 / greater, a.1 / greater);
}

pub fn times(vector: Vector2, scalar: i32) -> Vector2 {
    return (vector.0 * scalar, vector.1 * scalar);
}
