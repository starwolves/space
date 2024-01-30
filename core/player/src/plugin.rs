use crate::boarding::{grab_mouse_on_board, player_boarded, PlayerBoarded};
use crate::configuration::{
    client_receive_pawnid, finished_configuration, server_new_client_configuration, Boarded,
};
use crate::connections::{
    buffer_server_events, clear_buffer, process_response, Accounts, AuthidI,
    SendServerConfiguration, ServerEventBuffer,
};
use crate::debug_camera::{spawn_debug_camera, ActivateDebugCamera};
use crate::names::UsedNames;
use crate::net::PlayerServerMessage;
use crate::{
    boarding::{done_boarding, BoardingAnnouncements},
    connections::{server_events, PlayerAwaitingBoarding},
};
use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate as BevyPreUpdate, SystemSet};
use entity::despawn::DespawnEntitySet;
use networking::messaging::{register_reliable_message, MessageSender};
use resources::modes::is_server_mode;
use resources::ordering::{PostUpdate, Update};
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
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    process_response.before(DespawnEntitySet),
                    server_events
                        .before(ConfigurationLabel::SpawnEntity)
                        .after(process_response),
                ),
            )
            .add_event::<SendServerConfiguration>()
            .add_systems(BevyPreUpdate, buffer_server_events)
            .add_systems(PostUpdate, (clear_buffer,))
            .add_systems(
                Update,
                (
                    done_boarding,
                    server_new_client_configuration
                        .in_set(ConfigurationLabel::SpawnEntity)
                        .before(ConfigurationLabel::Main)
                        .after(process_response),
                    finished_configuration
                        .after(ConfigurationLabel::Main)
                        .after(process_response),
                    player_boarded,
                ),
            )
            .init_resource::<AuthidI>()
            .init_resource::<BoardingAnnouncements>()
            .add_event::<PlayerAwaitingBoarding>()
            .init_resource::<Accounts>()
            .add_event::<PlayerBoarded>()
            .init_resource::<ServerEventBuffer>()
            .init_resource::<UsedNames>();
        } else {
            app.add_systems(
                Update,
                (
                    client_receive_pawnid,
                    spawn_debug_camera,
                    grab_mouse_on_board.before(CursorSet::Perform),
                ),
            )
            .init_resource::<Boarded>()
            .add_event::<ActivateDebugCamera>();
        }
        register_reliable_message::<PlayerServerMessage>(app, MessageSender::Server, true);
    }
}
