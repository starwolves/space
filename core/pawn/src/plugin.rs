use std::env;

use crate::actions::{build_actions, examine, examine_prerequisite_check};
use bevy::prelude::IntoSystemDescriptor;
use bevy::prelude::{App, Plugin};
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
            .add_system(
                build_actions
                    .label(ActionsLabels::Build)
                    .after(ActionsLabels::Init),
            );
        }
    }
}
