use bevy_math::Vec3;
use bevy_rapier3d::{
    na::Quaternion,
    rapier::math::{Isometry, Real, Rotation, Translation},
};
use bevy_transform::components::Transform;

pub fn transform_to_isometry(transform: Transform) -> Isometry<Real> {
    let translation: Translation<f32> = Vec3::new(
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
    )
    .into();

    let quaternion = Quaternion::new(
        transform.rotation.w,
        transform.rotation.x,
        transform.rotation.y,
        transform.rotation.z,
    );

    let rotation = Rotation::from_quaternion(quaternion);

    Isometry::<Real> {
        translation: translation,
        rotation: rotation,
    }
}
