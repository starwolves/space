use crate::{
    actions::{build_actions, set_action_header_name},
    examine::{
        examine_entity, finalize_examine_entity, finalize_examine_map, NetConnExamine, NetExamine,
    },
};
use api::data::{ActionsLabels, PostUpdateLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use bevy::{app::CoreStage::PostUpdate, prelude::SystemSet};
use networking::messages::net_system;

pub struct ExaminablePlugin;
impl Plugin for ExaminablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            build_actions
                .label(ActionsLabels::Build)
                .after(ActionsLabels::Init),
        )
        .add_system(
            set_action_header_name
                .after(ActionsLabels::Build)
                .before(ActionsLabels::Approve),
        )
        .add_system_to_stage(
            PostUpdate,
            finalize_examine_map.before(PostUpdateLabels::EntityUpdate),
        )
        .add_event::<NetExamine>()
        .add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .after(PostUpdateLabels::VisibleChecker)
                .label(PostUpdateLabels::Net)
                .with_system(net_system::<NetExamine>)
                .with_system(net_system::<NetConnExamine>),
        )
        .add_event::<NetConnExamine>()
        .add_system_to_stage(
            PostUpdate,
            finalize_examine_entity.before(PostUpdateLabels::EntityUpdate),
        )
        .add_system(examine_entity.after(ActionsLabels::Action));
    }
}
