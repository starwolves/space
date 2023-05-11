use crate::account::{account_verification, Accounts};
use crate::boarding::{player_boarded, PlayerBoarded, SpawnPoints};
use crate::configuration::{
    client_receive_pawnid, finished_configuration, server_new_client_configuration, Boarded,
};
use crate::connections::{process_response, AuthidI, SendServerConfiguration};
use crate::debug_camera::spawn_debug_camera;
use crate::net::PlayerServerMessage;
use crate::{
    boarding::{done_boarding, BoardingAnnouncements, InputUIInputTransmitText},
    connections::{server_events, PlayerAwaitingBoarding},
};
use bevy::prelude::IntoSystemConfig;
use bevy::prelude::{App, Plugin, SystemSet};
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
                .add_system(done_boarding)
                .init_resource::<AuthidI>()
                .init_resource::<BoardingAnnouncements>()
                .add_event::<InputUIInputTransmitText>()
                .add_event::<PlayerAwaitingBoarding>()
                .add_system(
                    server_new_client_configuration
                        .in_set(ConfigurationLabel::SpawnEntity)
                        .before(ConfigurationLabel::Main),
                )
                .add_event::<InputUIInputTransmitText>()
                .add_system(finished_configuration.after(ConfigurationLabel::Main))
                .add_system(server_events.before(ConfigurationLabel::SpawnEntity))
                .add_system(process_response)
                .add_system(account_verification)
                .init_resource::<Accounts>()
                .add_event::<PlayerBoarded>()
                .add_system(player_boarded);
        } else {
            app.add_system(client_receive_pawnid)
                .add_system(spawn_debug_camera)
                .add_plugin(LookTransformPlugin)
                .add_plugin(FpsCameraPlugin::default())
                .init_resource::<Boarded>();
        }
        app.init_resource::<SpawnPoints>();
        register_reliable_message::<PlayerServerMessage>(app, MessageSender::Server);
    }
}
