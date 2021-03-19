
use bevy::{math::Vec3, prelude::Color};
use bevy::math::Quat;

pub fn string_color_to_color(string_color : &str) -> Color {

    let string_values : Vec<&str> = string_color.split(",").collect();

    let mut red_color = 0.;
    let mut green_color = 0.;
    let mut blue_color = 0.;
    let mut alpha_color = 0.;

    let mut i : u8 = 0;
    for string_value  in string_values {
        match i {
            0 => {red_color = string_value.parse::<f32>().unwrap();},
            1 => {green_color = string_value.parse::<f32>().unwrap();},
            2 => {blue_color = string_value.parse::<f32>().unwrap();},
            3 => {alpha_color = string_value.parse::<f32>().unwrap();},
            _ => ()
        }

        i+=1;
    };

    Color::rgba(red_color,green_color,blue_color,alpha_color)

}



pub fn string_quat_to_quat(string_quad : &str) -> Quat {

    let new_string = string_quad.replace(&['(', ')',' '][..], "");

    let string_values : Vec<&str> = new_string.split(",").collect();

    let mut x = 0.;
    let mut y = 0.;
    let mut z = 0.;
    let mut w = 0.;

    let mut i : u8 = 0;

    for string_value  in string_values {
        match i {
            0 => {x = string_value.parse::<f32>().unwrap();},
            1 => {y = string_value.parse::<f32>().unwrap();},
            2 => {z = string_value.parse::<f32>().unwrap();},
            3 => {w = string_value.parse::<f32>().unwrap();},
            _ => ()
        }

        i+=1;
    };

    Quat::from_xyzw(x,y,z,w)

}

const STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE : &str = "main.rs json_vec3_to_vec3() Error cannot parse cell id string as Vector 3.";

pub fn json_vec3_to_vec3(string_vector : &str) -> Vec3 {

   

    let mut split_result : Vec<&str> = string_vector.split("(").collect();

    let mut new_string : &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    return Vec3::new(
        split_result[0].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[1].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[2].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE)
    );

}
