use std::time::Duration;

use crate::actions::{build_actions, examine, examine_prerequisite_check};
use crate::camera::{
    clear_mouse_stamps, client_sync_look_transform, server_sync_look_transform, LookTransformSet,
    MouseInputStamps,
};
use crate::net::UnreliableControllerClientMessage;
use bevy::app::Update;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use bevy::time::common_conditions::on_timer;
use bevy_renet::renet::RenetClient;
use networking::messaging::{register_unreliable_message, MessageSender};
use resources::modes::{is_correction_mode, is_server_mode};
use resources::sets::{ActionsSet, MainSet};
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            if !is_correction_mode(app) {
                app.add_systems(
                    FixedUpdate,
                    (
                        examine_prerequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Init),
                        examine
                            .in_set(ActionsSet::Action)
                            .after(ActionsSet::Approve),
                        build_actions
                            .in_set(ActionsSet::Build)
                            .after(ActionsSet::Init),
                        server_sync_look_transform.in_set(LookTransformSet::Sync),
                    )
                        .in_set(MainSet::Update),
                );
            }
            app.add_systems(
                FixedUpdate,
                server_sync_look_transform
                    .in_set(LookTransformSet::Sync)
                    .in_set(MainSet::Update),
            );
        } else {
            app.add_systems(
                Update,
                client_sync_look_transform
                    .run_if(resource_exists::<RenetClient>())
                    .in_set(MainSet::Update)
                    .in_set(LookTransformSet::Sync)
                    .run_if(on_timer(Duration::from_secs_f32(1. / 60.))),
            )
            .init_resource::<MouseInputStamps>()
            .add_systems(
                FixedUpdate,
                (clear_mouse_stamps.in_set(MainSet::PostUpdate),),
            );
        }
        register_unreliable_message::<UnreliableControllerClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}
