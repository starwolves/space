use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::server::net_system;
use server_instance::labels::{MapLabels, PostUpdateLabels};

use crate::{map::MapHolders, map_input::MapData};

use super::{
    change_overlay::change_map_overlay,
    map_input::{map_input, request_map_overlay, NetRequestOverlay},
};
use bevy::app::CoreStage::PostUpdate;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "server") {
            app.init_resource::<MapData>()
                .add_event::<NetRequestOverlay>()
                .add_system(change_map_overlay.label(MapLabels::ChangeMode))
                .add_system(request_map_overlay)
                .add_system(map_input.label(MapLabels::ChangeMode))
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetRequestOverlay>),
                )
                .init_resource::<MapHolders>();
        }
    }
}
