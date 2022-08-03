use api::data::{MapLabels, PostUpdateLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;

use crate::{map::MapHolders, map_input::MapData};

use super::{
    change_display_mode::change_display_mode,
    map_input::{map_input, request_display_modes, NetRequestDisplayModes},
};
use bevy::app::CoreStage::PostUpdate;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapData>()
            .add_event::<NetRequestDisplayModes>()
            .add_system(change_display_mode.label(MapLabels::ChangeMode))
            .add_system(request_display_modes)
            .add_system(map_input.label(MapLabels::ChangeMode))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetRequestDisplayModes>),
            )
            .init_resource::<MapHolders>();
    }
}
