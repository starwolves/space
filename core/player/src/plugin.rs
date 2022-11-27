use std::env;

use crate::{
    boarding::{done_boarding, on_boarding, BoardingAnnouncements, InputUIInputTransmitText},
    connection::{AuthidI, SendServerConfiguration},
    connections::{configure, finished_configuration, server_events, PlayerAwaitingBoarding},
};
use bevy::prelude::IntoSystemDescriptor;
use bevy::prelude::{App, Plugin, SystemLabel};
use networking::server::HandleToEntity;

/// Atmospherics systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConfigurationLabel {
    SpawnEntity,
    Main,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<SendServerConfiguration>()
                .init_resource::<HandleToEntity>()
                .add_system(done_boarding)
                .add_system(on_boarding)
                .init_resource::<AuthidI>()
                .init_resource::<BoardingAnnouncements>()
                .add_event::<InputUIInputTransmitText>()
                .add_event::<PlayerAwaitingBoarding>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::SpawnEntity)
                        .before(ConfigurationLabel::Main),
                )
                .add_event::<InputUIInputTransmitText>()
                .add_system(finished_configuration.after(ConfigurationLabel::Main))
                .add_system(server_events.before(ConfigurationLabel::SpawnEntity));
        }
    }
}
