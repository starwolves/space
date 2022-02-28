use bevy_internal::{
    math::{Mat3, Quat, Vec3},
    prelude::Transform,
};

use crate::space::core::gridmap::resources::Vec2Int;

pub fn string_color_to_color(string_color: &str) -> (f32, f32, f32, f32) {
    let string_values: Vec<&str> = string_color.split(",").collect();

    let mut red_color = 0.;
    let mut green_color = 0.;
    let mut blue_color = 0.;
    let mut alpha_color = 0.;

    let mut i: u8 = 0;
    for string_value in string_values {
        match i {
            0 => {
                red_color = string_value.parse::<f32>().unwrap();
            }
            1 => {
                green_color = string_value.parse::<f32>().unwrap();
            }
            2 => {
                blue_color = string_value.parse::<f32>().unwrap();
            }
            3 => {
                alpha_color = string_value.parse::<f32>().unwrap();
            }
            _ => (),
        }

        i += 1;
    }

    (red_color, green_color, blue_color, alpha_color)
}

pub fn string_quat_to_quat(string_quad: &str) -> Quat {
    let new_string = string_quad.replace(&['(', ')', ' '][..], "");

    let string_values: Vec<&str> = new_string.split(",").collect();

    let mut x = 0.;
    let mut y = 0.;
    let mut z = 0.;
    let mut w = 0.;

    let mut i: u8 = 0;

    for string_value in string_values {
        match i {
            0 => {
                x = string_value.parse::<f32>().unwrap();
            }
            1 => {
                y = string_value.parse::<f32>().unwrap();
            }
            2 => {
                z = string_value.parse::<f32>().unwrap();
            }
            3 => {
                w = string_value.parse::<f32>().unwrap();
            }
            _ => (),
        }

        i += 1;
    }

    Quat::from_xyzw(x, y, z, w)
}

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
