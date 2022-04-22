use bevy_math::{Vec2, Vec3};

pub fn get_vector(target: Vec3, current: Vec3) -> Vec2 {
    let x = target.x - current.x;
    let y = target.z - current.z;

    Vec2::new(x * -1., y).normalize_or_zero()
}

pub fn get_proximity(target: Vec3, current: Vec3) -> f32 {
    let x_dist = (target.x - current.x).abs();
    let y_dist = (target.y - current.y).abs();
    x_dist + y_dist
}
