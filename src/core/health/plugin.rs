use crate::core::{combat::attack::Attack, space_plugin::plugin::PostUpdateLabels};

use super::{
    health::{health_ui_update, ClientHealthUICache},
    net::{net_system, NetHealthUpdate},
};
use bevy::{
    app::CoreStage::PostUpdate,
    prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientHealthUICache>()
            .add_event::<NetHealthUpdate>()
            .add_event::<Attack>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(health_ui_update),
            )
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
