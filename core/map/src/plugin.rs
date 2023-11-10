use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender,
};
use player::{connections::process_response, plugin::ConfigurationLabel};
use resources::{
    modes::is_server_mode,
    sets::{MainSet, MapSet},
};

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
        if is_server_mode(app) {
            app.init_resource::<MapData>()
                .add_systems(
                    FixedUpdate,
                    (
                        request_map_overlay,
                        change_map_overlay.in_set(MapSet::ChangeMode),
                        map_input.in_set(MapSet::ChangeMode),
                        configure
                            .in_set(ConfigurationLabel::Main)
                            .after(ConfigurationLabel::SpawnEntity)
                            .after(process_response),
                    )
                        .in_set(MainSet::Update),
                )
                .add_systems(FixedUpdate, incoming_messages.in_set(MainSet::PreUpdate))
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
