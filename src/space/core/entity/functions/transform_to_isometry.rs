use bevy::prelude::Transform;
use bevy_rapier3d::{
    na::{Quaternion, Translation3},
    rapier::math::{Isometry, Real, Rotation},
};

pub fn transform_to_isometry(transform: Transform) -> Isometry<Real> {
    let translation = Translation3::new(
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
    );

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
