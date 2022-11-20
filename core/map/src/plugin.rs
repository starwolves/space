use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use networking::server::net_system;
use player::plugin::ConfigurationLabel;
use resources::labels::{MapLabels, PostUpdateLabels};

use crate::{
    connections::{configure, NetConfig},
    map::MapHolders,
    map_input::{InputMap, InputMapChangeDisplayMode, InputMapRequestOverlay, MapData},
    networking::incoming_messages,
};

use super::{
    change_overlay::change_map_overlay,
    map_input::{map_input, request_map_overlay, NetRequestOverlay},
};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
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
                        .with_system(net_system::<NetRequestOverlay>)
                        .with_system(net_system::<NetConfig>),
                )
                .init_resource::<MapHolders>()
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputMapChangeDisplayMode>()
                .add_event::<InputMap>()
                .add_event::<InputMapRequestOverlay>()
                .add_event::<NetConfig>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
    }
}
