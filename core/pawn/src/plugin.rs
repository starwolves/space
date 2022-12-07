use std::env;

use crate::{
    actions::{examine, examine_prerequisite_check},
    networking::{incoming_messages, PawnClientMessage},
    pawn::account_name,
};
use bevy::app::CoreStage::PreUpdate;
use bevy::prelude::IntoSystemDescriptor;
use bevy::prelude::{App, Plugin};
use networking::messaging::{init_reliable_message, MessageSender};
use player::names::InputAccountName;
use resources::labels::ActionsLabels;
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
            .add_system_to_stage(PreUpdate, incoming_messages)
            .add_event::<InputAccountName>();
        }

        init_reliable_message::<PawnClientMessage>(app, MessageSender::Client);
    }
}
