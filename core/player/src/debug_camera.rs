use bevy::{
    core_pipeline::fxaa::Fxaa,
    prelude::{Camera, Camera3dBundle, Commands, EventReader, Res, ResMut, Vec3},
};

use cameras::controllers::fps::{ActiveCamera, FpsCameraBundle, FpsCameraController};
use graphics::settings::GraphicsSettings;
use networking::client::IncomingReliableServerMessage;

use crate::net::PlayerServerMessage;
use bevy::prelude::Local;

/// Spawn 3D debug camera on boarding.

pub(crate) fn spawn_debug_camera(
    mut commands: Commands,
    mut messages: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut spawning: Local<bool>,
    mut state: ResMut<ActiveCamera>,
    settings: Res<GraphicsSettings>,
) {
    // Skip one frame to prevent camera ambiguity.
    if *spawning {
        *spawning = false;
        let id = commands
            .spawn(Camera3dBundle {
                camera: Camera {
                    msaa_writeback: settings.msaa.is_enabled(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(FpsCameraBundle::new(
                FpsCameraController::default(),
                Vec3::new(0., 1.8, 0.),
                Vec3::new(0., 1.8, -2.),
                Vec3::Y,
            ))
            // .insert(AtmosphereCamera::default())
            .insert(Fxaa {
                enabled: settings.fxaa.is_some(),
                ..Default::default()
            })
            .id();

        state.option = Some(id);
    }

    for message in messages.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                *spawning = true;
            }
            _ => {}
        }
    }
}
