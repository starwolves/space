use crate::actions::{build_actions, examine, examine_prerequisite_check};
use crate::camera::{
    clear_mouse_stamps, clear_mouse_stamps_server, client_sync_look_transform,
    server_sync_look_transform, LookTransformSet, MouseInputStamps, ServerMouseInputStamps,
};
use crate::net::UnreliableControllerClientMessage;
use bevy::app::Update;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use bevy_renet::renet::RenetClient;
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
            )
            .init_resource::<ServerMouseInputStamps>()
            .add_systems(
                FixedUpdate,
                clear_mouse_stamps_server.in_set(MainSet::PostUpdate),
            );
        } else {
            app.add_systems(
                Update,
                client_sync_look_transform
                    .run_if(resource_exists::<RenetClient>())
                    .in_set(MainSet::Update)
                    .in_set(LookTransformSet::Sync),
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
