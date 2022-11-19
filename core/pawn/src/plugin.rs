use std::env;

use crate::{
    actions::{examine, examine_prerequisite_check},
    examine_events::NetPawn,
    networking::incoming_messages,
    pawn::account_name,
};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::server::net_system;
use player::names::InputAccountName;
use resources::labels::{ActionsLabels, PostUpdateLabels, PreUpdateLabels};
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(
                examine_prerequisite_check
                    .label(ActionsLabels::Approve)
                    .after(ActionsLabels::Init),
            )
            .add_system(
                examine
                    .label(ActionsLabels::Action)
                    .after(ActionsLabels::Approve),
            )
            .add_system(account_name)
            .add_event::<NetPawn>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetPawn>),
            )
            .add_system_to_stage(
                PreUpdate,
                incoming_messages
                    .after(PreUpdateLabels::NetEvents)
                    .label(PreUpdateLabels::ProcessInput),
            )
            .add_event::<InputAccountName>();
        }
    }
}
