use bevy_math::{Quat, Vec3};
use bevy_rapier3d::rapier::math::{Isometry, Real};
use bevy_transform::components::Transform;

pub fn isometry_to_transform(isometry: Isometry<Real>) -> Transform {
    let translation = Vec3::new(
        isometry.translation.x,
        isometry.translation.y,
        isometry.translation.z,
    );

    let rotation = Quat::from_xyzw(
        isometry.rotation.i,
        isometry.rotation.j,
        isometry.rotation.k,
        isometry.rotation.w,
    );

    Transform {
        translation: translation,
        rotation: rotation,
        scale: Vec3::new(1., 1., 1.),
    }
}
