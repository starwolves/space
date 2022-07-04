use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};

use crate::core::space_plugin::plugin::{MapLabels, PostUpdateLabels};

use super::{
    change_display_mode::change_display_mode,
    map_input::{
        map_input, request_display_modes, InputMap, InputMapChangeDisplayMode,
        InputMapRequestDisplayModes, NetRequestDisplayModes,
    },
    map_overlay::MapData,
    net::net_system,
};
use bevy::app::CoreStage::PostUpdate;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapData>()
            .add_event::<InputMapChangeDisplayMode>()
            .add_event::<InputMapRequestDisplayModes>()
            .add_event::<NetRequestDisplayModes>()
            .add_event::<InputMap>()
            .add_system(change_display_mode.label(MapLabels::ChangeMode))
            .add_system(request_display_modes)
            .add_system(map_input.label(MapLabels::ChangeMode))
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
