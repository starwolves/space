use crate::actions::{build_actions, examine, examine_prerequisite_check};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use resources::is_server::is_server;
use resources::sets::{ActionsSet, MainSet};
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
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
                )
                    .in_set(MainSet::Update),
            );
        }
    }
}
