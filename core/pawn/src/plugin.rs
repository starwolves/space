use crate::actions::{build_actions, examine, examine_prerequisite_check};
use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};
use resources::is_server::is_server;
use resources::labels::ActionsLabels;
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                Update,
                (
                    examine_prerequisite_check
                        .in_set(ActionsLabels::Approve)
                        .after(ActionsLabels::Init),
                    examine
                        .in_set(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                    build_actions
                        .in_set(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                ),
            );
        }
    }
}
