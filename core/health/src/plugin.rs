use bevy::{
    app::CoreStage::PostUpdate,
    prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
};
use networking::messages::net_system;
use api::{
    data::PostUpdateLabels,
    examinable::ExamineLabels,
    health::{NetHealth, NetHealthUpdate},
};

use crate::examine_events::{examine_entity, examine_map};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .after(PostUpdateLabels::VisibleChecker)
                .label(PostUpdateLabels::Net)
                .with_system(net_system::<NetHealthUpdate>)
                .with_system(net_system::<NetHealth>),
        )
        .add_event::<NetHealthUpdate>()
        .add_event::<NetHealth>()
        .add_system(examine_map.after(ExamineLabels::Default))
        .add_system(examine_entity.after(ExamineLabels::Default));
    }
}
