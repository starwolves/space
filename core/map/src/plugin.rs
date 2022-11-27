use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use player::plugin::ConfigurationLabel;
use resources::labels::MapLabels;

use crate::{
    connections::configure,
    map::MapHolders,
    map_input::{InputMap, InputMapChangeDisplayMode, InputMapRequestOverlay, MapData},
    networking::incoming_messages,
};

use super::{
    change_overlay::change_map_overlay,
    map_input::{map_input, request_map_overlay},
};
use bevy::app::CoreStage::PreUpdate;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<MapData>()
                .add_system(change_map_overlay.label(MapLabels::ChangeMode))
                .add_system(request_map_overlay)
                .add_system(map_input.label(MapLabels::ChangeMode))
                .init_resource::<MapHolders>()
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputMapChangeDisplayMode>()
                .add_event::<InputMap>()
                .add_event::<InputMapRequestOverlay>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
    }
}
