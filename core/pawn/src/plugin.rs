use crate::actions::{build_actions, examine, examine_prerequisite_check};
use bevy::prelude::{App, IntoSystemConfig, Plugin};
use resources::is_server::is_server;
use resources::labels::ActionsLabels;
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                examine_prerequisite_check
                    .in_set(ActionsLabels::Approve)
                    .after(ActionsLabels::Init),
            )
            .add_system(
                examine
                    .in_set(ActionsLabels::Action)
                    .after(ActionsLabels::Approve),
            )
            .add_system(
                build_actions
                    .in_set(ActionsLabels::Build)
                    .after(ActionsLabels::Init),
            );
        }
    }
}
