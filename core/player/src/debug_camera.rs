use bevy::prelude::{Camera3dBundle, Commands, EventReader, Transform, Vec3};

use networking::client::IncomingReliableServerMessage;

use crate::net::PlayerServerMessage;
use bevy::prelude::Local;

/// Spawn 3D debug camera on boarding.

pub(crate) fn spawn_debug_camera(
    mut commands: Commands,
    mut messages: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut spawning: Local<bool>,
) {
    // Skip one frame to prevent camera ambiguity.
    if *spawning {
        *spawning = false;
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0., 1.8, 0.)
                .looking_at(Vec3::new(0., 1.8, -2.), Vec3::Y),
            ..Default::default()
        });
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
