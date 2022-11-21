use std::env;

use crate::{
    boarding::{
        done_boarding, on_boarding, BoardingAnnouncements, InputUIInputTransmitText, NetOnBoarding,
    },
    connection::{AuthidI, NetPlayerConn, SendServerConfiguration},
    connections::{configure, finished_configuration, server_events, PlayerAwaitingBoarding},
    networking::NetDoneBoarding,
};
use bevy::prelude::{App, Plugin, SystemLabel, SystemSet};
use bevy::{app::CoreStage::PostUpdate, prelude::IntoSystemDescriptor};
use networking::server::{net_system, HandleToEntity};
use resources::labels::PostUpdateLabels;

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
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetOnBoarding>)
                        .with_system(net_system::<NetDoneBoarding>)
                        .with_system(net_system::<NetPlayerConn>),
                )
                .add_event::<NetOnBoarding>()
                .add_event::<NetDoneBoarding>()
                .init_resource::<AuthidI>()
                .init_resource::<BoardingAnnouncements>()
                .add_event::<NetPlayerConn>()
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
