use api::data::{ActionsLabels, PostUpdateLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;

use crate::{
    actions::{examine, examine_prerequisite_check},
    examine_events::examine_map,
    pawn::UsedNames,
    user_name::NetPawn,
};

use super::user_name::user_name;
use bevy::app::CoreStage::PostUpdate;
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsedNames>()
            .add_system(user_name)
            .add_system(examine_map.after(ActionsLabels::Action))
            .add_event::<NetPawn>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetPawn>),
            )
            .add_system(
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
