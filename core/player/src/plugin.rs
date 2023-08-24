use crate::boarding::{grab_mouse_on_board, player_boarded, PlayerBoarded, SpawnPoints};
use crate::configuration::{
    client_receive_pawnid, finished_configuration, server_new_client_configuration, Boarded,
};
use crate::connections::{
    buffer_server_events, clear_buffer, process_response, Accounts, AuthidI,
    SendServerConfiguration, ServerEventBuffer,
};
use crate::debug_camera::{spawn_debug_camera, ActivateDebugCamera};
use crate::net::PlayerServerMessage;
use crate::{
    boarding::{done_boarding, BoardingAnnouncements, InputUIInputTransmitText},
    connections::{server_events, PlayerAwaitingBoarding},
};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, SystemSet, Update};
use cameras::controllers::fps::FpsCameraPlugin;
use cameras::LookTransformPlugin;
use networking::{
    messaging::{register_reliable_message, MessageSender},
    server::HandleToEntity,
};
use resources::is_server::is_server;
use resources::sets::MainSet;
use ui::cursor::CursorSet;

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
                .add_systems(Update, buffer_server_events)
                .add_systems(FixedUpdate, (clear_buffer.in_set(MainSet::PostUpdate),))
                .add_systems(
                    FixedUpdate,
                    (
                        done_boarding,
                        server_new_client_configuration
                            .in_set(ConfigurationLabel::SpawnEntity)
                            .before(ConfigurationLabel::Main)
                            .after(process_response),
                        finished_configuration
                            .after(ConfigurationLabel::Main)
                            .after(process_response),
                        server_events
                            .after(buffer_server_events)
                            .before(ConfigurationLabel::SpawnEntity)
                            .after(process_response),
                        process_response,
                        player_boarded,
                    )
                        .in_set(MainSet::Update),
                )
                .init_resource::<AuthidI>()
                .init_resource::<BoardingAnnouncements>()
                .add_event::<InputUIInputTransmitText>()
                .add_event::<PlayerAwaitingBoarding>()
                .add_event::<InputUIInputTransmitText>()
                .init_resource::<Accounts>()
                .add_event::<PlayerBoarded>()
                .init_resource::<ServerEventBuffer>();
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    client_receive_pawnid,
                    spawn_debug_camera,
                    grab_mouse_on_board.before(CursorSet::Perform),
                )
                    .in_set(MainSet::Update),
            )
            .add_plugins(LookTransformPlugin)
            .add_plugins(FpsCameraPlugin::default())
            .init_resource::<Boarded>()
            .add_event::<ActivateDebugCamera>();
        }
        app.init_resource::<SpawnPoints>();
        register_reliable_message::<PlayerServerMessage>(app, MessageSender::Server);
    }
}
