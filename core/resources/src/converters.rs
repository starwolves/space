use bevy::{
    math::{Quat, Vec3},
    prelude::{Mat3, Transform},
};

use crate::math::Vec2Int;

/// Error message for vector3.
const STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_vec3_to_vec3() Error cannot parse cell id string as Vector 3.";

/// Convert vector3.
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

/// Error message for vector2.
const _STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_vec2_to_vec2() Error cannot parse cell id string as Vector 3.";

/// Convert vector2.
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

const STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_transform_to_transform() Error cannot parse floats of transform.";

pub fn string_transform_to_transform(string_transform: &str) -> Transform {
    let mut split_result: Vec<&str> = string_transform.split(",").collect();

    let mut odd_index_value: usize = 0;

    for seperated_value in split_result.iter() {
        if seperated_value.contains(" - ") {
            let odd_values: Vec<&str> = seperated_value.split(" - ").collect();

            split_result.remove(odd_index_value);

            split_result.insert(8, odd_values[0]);
            split_result.insert(9, odd_values[1]);

            break;
        }

        odd_index_value += 1;
    }

    let mut current_index: usize = 0;

    let mut clean_strings: Vec<String> = Vec::new();

    for seperated_value in split_result.iter() {
        clean_strings.insert(current_index, seperated_value.replace(' ', ""));

        current_index += 1;
    }

    let translation = Vec3::new(
        clean_strings[9]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[10]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[11]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let basis_x = Vec3::new(
        clean_strings[0]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[3]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[6]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let basis_y = Vec3::new(
        clean_strings[1]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[4]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[7]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let basis_z = Vec3::new(
        clean_strings[2]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[5]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[8]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let mut transform_result = Transform::from_translation(translation);
    transform_result.rotation = Quat::from_mat3(&Mat3::from_cols(basis_x, basis_y, basis_z));
    // Hard coding scale, not required for current use case
    transform_result.scale = Vec3::new(1., 1., 1.);

    transform_result
}
