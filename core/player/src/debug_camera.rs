use bevy::{
    core_pipeline::{fxaa::Fxaa, tonemapping::Tonemapping, Skybox},
    prelude::{
        Camera, Camera3dBundle, Commands, Event, EventReader, Res, ResMut, Vec3, VisibilityBundle,
    },
};

use cameras::controllers::fps::{ActiveCamera, FpsCameraBundle, FpsCameraController};
use graphics::{settings::PerformanceSettings, skybox::SkyboxHandle};

use bevy::prelude::Local;
use resources::pawn::HUMANOID_HEIGHT;

#[derive(Event)]
pub struct ActivateDebugCamera;

/// Spawn 3D debug camera on boarding.
/// ONLY MANUALLY ENABLED FOR DEBUGGING. NOT ACTIVELY USED.
pub(crate) fn spawn_debug_camera(
    mut commands: Commands,
    mut ativates: EventReader<ActivateDebugCamera>,
    mut spawning: Local<bool>,
    mut state: ResMut<ActiveCamera>,
    settings: Res<PerformanceSettings>,
    handle: Res<SkyboxHandle>,
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
                tonemapping: Tonemapping::ReinhardLuminance,
                ..Default::default()
            })
            .insert(Skybox {
                image: handle.h.clone_weak(),
                brightness: 1.,
            })
            .insert(FpsCameraBundle::new(
                FpsCameraController::default(),
                Vec3::new(0., HUMANOID_HEIGHT, 0.),
                Vec3::new(0., HUMANOID_HEIGHT, -2.),
                Vec3::Y,
            ))
            // .insert(AtmosphereCamera::default())
            .insert(Fxaa {
                enabled: settings.fxaa.is_some(),
                ..Default::default()
            })
            .insert(VisibilityBundle::default())
            .id();

        state.option = Some(id);
    }

    for _ in ativates.read() {
        *spawning = true;
    }
}
