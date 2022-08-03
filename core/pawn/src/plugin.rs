use api::{data::PostUpdateLabels, examinable::ExamineLabels, tab_actions::TabActionsQueueLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;

use crate::{examine_events::examine_map, pawn::UsedNames, user_name::NetPawn};

use super::{actions::actions, user_name::user_name};
use bevy::app::CoreStage::PostUpdate;
pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsedNames>()
            .add_system(user_name)
            .add_system(examine_map.after(ExamineLabels::Default))
            .add_system(
                actions
                    .label(ExamineLabels::Start)
                    .after(TabActionsQueueLabels::TabAction),
            )
            .add_event::<NetPawn>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetPawn>),
            );
    }
}
