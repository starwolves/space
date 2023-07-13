use crate::boarding::{player_boarded, PlayerBoarded, SpawnPoints};
use crate::configuration::{
    client_receive_pawnid, finished_configuration, server_new_client_configuration, Boarded,
};
use crate::connections::{process_response, Accounts, AuthidI, SendServerConfiguration};
use crate::debug_camera::spawn_debug_camera;
use crate::net::PlayerServerMessage;
use crate::{
    boarding::{done_boarding, BoardingAnnouncements, InputUIInputTransmitText},
    connections::{server_events, PlayerAwaitingBoarding},
};
use bevy::prelude::{App, IntoSystemConfigs, Plugin, SystemSet, Update};
use cameras::controllers::fps::FpsCameraPlugin;
use cameras::LookTransformPlugin;
use networking::{
    messaging::{register_reliable_message, MessageSender},
    server::HandleToEntity,
};
use resources::is_server::is_server;

/// Atmospherics systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ConfigurationLabel {
    SpawnEntity,
    Main,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<SendServerConfiguration>()
                .init_resource::<HandleToEntity>()
                .add_systems(
                    Update,
                    (
                        done_boarding,
                        server_new_client_configuration
                            .in_set(ConfigurationLabel::SpawnEntity)
                            .before(ConfigurationLabel::Main),
                        finished_configuration.after(ConfigurationLabel::Main),
                        server_events.before(ConfigurationLabel::SpawnEntity),
                        process_response,
                        player_boarded,
                    ),
                )
                .init_resource::<AuthidI>()
                .init_resource::<BoardingAnnouncements>()
                .add_event::<InputUIInputTransmitText>()
                .add_event::<PlayerAwaitingBoarding>()
                .add_event::<InputUIInputTransmitText>()
                .init_resource::<Accounts>()
                .add_event::<PlayerBoarded>();
        } else {
            app.add_systems(Update, (client_receive_pawnid, spawn_debug_camera))
                .add_plugins(LookTransformPlugin)
                .add_plugins(FpsCameraPlugin::default())
                .init_resource::<Boarded>();
        }
        app.init_resource::<SpawnPoints>();
        register_reliable_message::<PlayerServerMessage>(app, MessageSender::Server);
    }
}
