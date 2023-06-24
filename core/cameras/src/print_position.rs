use bevy::prelude::{Query, Transform, With};

use crate::controllers::fps::FpsCameraController;

#[allow(dead_code)]
pub fn print_camera_position(cameras: Query<&Transform, With<FpsCameraController>>) {
    for cam in cameras.iter() {
        bevy::log::info!("{:?}", cam.translation);
    }
}
