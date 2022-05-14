use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

use self::{
    events::{
        InputMap, InputMapChangeDisplayMode, InputMapRequestDisplayModes, NetRequestDisplayModes,
    },
    resources::MapData,
    systems::{
        change_display_mode::change_display_mode, map_input::map_input, net_system::net_system,
        request_display_modes::request_display_modes,
    },
};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

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
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }
}
use bevy_app::CoreStage::PostUpdate;

use super::{MapLabels, PostUpdateLabels};
