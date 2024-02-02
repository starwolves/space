use std::time::Duration;

use crate::actions::{build_actions, examine, examine_prerequisite_check};
use crate::camera::{clear_mouse_stamps, mouse_input, LookTransformSet, MouseInputStamps};
use crate::net::UnreliableControllerClientMessage;
use crate::pawn::SpawningPlayer;
use bevy::app::PreUpdate as BevyPreUpdate;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use bevy::time::common_conditions::on_timer;
use bevy_renet::renet::RenetClient;
use cameras::controllers::fps::control_system;
use networking::client::PreUpdateSendMessage;
use networking::messaging::{register_unreliable_message, MessageSender};
use resources::modes::is_server_mode;
use resources::ordering::{ActionsSet, PostUpdate, Update};
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
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
                ),
            )
            .add_event::<SpawningPlayer>();
        } else {
            app.add_systems(
                BevyPreUpdate,
                mouse_input
                    .before(PreUpdateSendMessage)
                    .after(control_system)
                    .run_if(resource_exists::<RenetClient>())
                    .in_set(LookTransformSet::Sync)
                    .run_if(on_timer(Duration::from_secs_f32(1. / 60.))),
            )
            .init_resource::<MouseInputStamps>()
            .add_systems(PostUpdate, (clear_mouse_stamps,));
        }
        register_unreliable_message::<UnreliableControllerClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}
