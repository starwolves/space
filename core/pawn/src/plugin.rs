use std::env;

use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use resources::labels::ActionsLabels;

use crate::actions::{examine, examine_prerequisite_check};

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
            );
        }
    }
}
