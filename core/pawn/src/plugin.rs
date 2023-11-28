use std::time::Duration;

use crate::actions::{build_actions, examine, examine_prerequisite_check};
use crate::camera::{client_sync_look_transform, server_sync_look_transform, LookTransformSet};
use crate::net::UnreliableControllerClientMessage;
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use bevy::time::common_conditions::on_timer;
use networking::messaging::{register_unreliable_message, MessageSender};
use resources::modes::is_server_mode;
use resources::sets::{ActionsSet, MainSet};
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
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
        } else {
            app.add_systems(
                FixedUpdate,
                client_sync_look_transform
                    .run_if(on_timer(Duration::from_secs_f32(5.)))
                    .in_set(MainSet::Update)
                    .in_set(LookTransformSet::Sync),
            );
        }
        register_unreliable_message::<UnreliableControllerClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}
