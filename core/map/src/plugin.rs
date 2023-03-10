use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
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
                .add_system(change_map_overlay.in_set(MapLabels::ChangeMode))
                .add_system(request_map_overlay)
                .add_system(map_input.in_set(MapLabels::ChangeMode))
                .init_resource::<MapHolders>()
                .add_system(incoming_messages.in_base_set(CoreSet::PreUpdate))
                .add_event::<InputMapChangeDisplayMode>()
                .add_event::<InputMap>()
                .add_event::<InputMapRequestOverlay>()
                .add_system(
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
        register_reliable_message::<MapServerMessage>(app, MessageSender::Server);
        register_unreliable_message::<MapUnreliableClientMessage>(app, MessageSender::Client);
        register_reliable_message::<MapReliableClientMessage>(app, MessageSender::Client);
    }
}
