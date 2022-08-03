const STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_vec3_to_vec3() Error cannot parse cell id string as Vector 3.";

pub fn string_vec3_to_vec3(string_vector: &str) -> Vec3 {
    let clean_string = string_vector.replace(" ", "");

    let mut split_result: Vec<&str> = clean_string.split("(").collect();

    let mut new_string: &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    Vec3::new(
        split_result[0]
            .parse::<f32>()
            .expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[1]
            .parse::<f32>()
            .expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[2]
            .parse::<f32>()
            .expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
    )
}

const _STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_vec2_to_vec2() Error cannot parse cell id string as Vector 3.";

pub fn _string_vec2_to_vec2_int(string_vector: &str) -> Vec2Int {
    let clean_string = string_vector.replace(" ", "");

    let mut split_result: Vec<&str> = clean_string.split("(").collect();

    let mut new_string: &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    Vec2Int {
        x: split_result[0]
            .parse::<f32>()
            .expect(_STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE) as i16,
        y: split_result[1]
            .parse::<f32>()
            .expect(_STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE) as i16,
    }
}

use api::data::Vec2Int;
use bevy::{
    math::{Quat, Vec3},
    prelude::Transform,
};
use bevy_rapier3d::{
    na::Quaternion,
    rapier::math::{Isometry, Real, Rotation, Translation},
};

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
