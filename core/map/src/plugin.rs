use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Update};
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender,
};
use player::plugin::ConfigurationLabel;
use resources::{is_server::is_server, labels::MapLabels};

use crate::{
    connections::configure,
    map::MapHolders,
    map_input::{
        incoming_messages, InputMap, InputMapChangeDisplayMode, InputMapRequestOverlay, MapData,
    },
    net::{MapReliableClientMessage, MapServerMessage, MapUnreliableClientMessage},
};

use super::{
    change_overlay::change_map_overlay,
    map_input::{map_input, request_map_overlay},
};
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.init_resource::<MapData>()
                .add_systems(
                    Update,
                    (
                        request_map_overlay,
                        change_map_overlay.in_set(MapLabels::ChangeMode),
                        map_input.in_set(MapLabels::ChangeMode),
                        configure
                            .in_set(ConfigurationLabel::Main)
                            .after(ConfigurationLabel::SpawnEntity),
                    ),
                )
                .add_systems(PreUpdate, incoming_messages)
                .add_event::<InputMapChangeDisplayMode>()
                .add_event::<InputMap>()
                .add_event::<InputMapRequestOverlay>()
                .init_resource::<MapHolders>();
        }
        register_reliable_message::<MapServerMessage>(app, MessageSender::Server);
        register_unreliable_message::<MapUnreliableClientMessage>(app, MessageSender::Client);
        register_reliable_message::<MapReliableClientMessage>(app, MessageSender::Client);
    }
}
