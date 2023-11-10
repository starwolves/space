use std::time::Duration;

use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, EventReader, Handle, NodeBundle, Query, Res,
        ResMut, Resource, TextBundle, With,
    },
    text::{Font, Text, TextSection, TextStyle},
    ui::{Style, Val},
};
use bevy_renet::renet::RenetClient;
use networking::client::IncomingReliableServerMessage;
use player::net::PlayerServerMessage;
use resources::hud::HudState;
use ui::fonts::{Fonts, ARIZONE_FONT};
#[derive(Component)]
pub struct ServerStats;

pub(crate) fn build_server_stats(state: Res<HudState>, mut commands: Commands, fonts: Res<Fonts>) {
    let arizone_font = fonts.handles.get(ARIZONE_FONT).unwrap();

    commands
        .entity(state.top_edge_node)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(5.),
                        height: Val::Percent(21.),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0., 0., 1., 0.2).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "",
                            TextStyle {
                                font: arizone_font.clone(),
                                font_size: 6.0,
                                color: Color::WHITE.into(),
                            },
                        ))
                        .insert(ServerStats);
                });
        });
}

#[derive(Resource, Default)]
pub struct ServerStatsState {
    pub connected_players: u16,
    pub rtt: f32,
}

impl ServerStatsState {
    pub fn to_sections(&self, font: Handle<Font>) -> Vec<TextSection> {
        let connected_section = TextSection {
            value: format!("{} connected.\n", self.connected_players),
            style: TextStyle {
                font: font.clone(),
                font_size: 6.,
                color: Color::WHITE.into(),
            },
        };

        let ping_section = TextSection {
            value: format!("{} ms.", Duration::from_secs_f32(self.rtt).as_millis()),
            style: TextStyle {
                font,
                font_size: 6.,
                color: Color::WHITE.into(),
            },
        };
        vec![connected_section, ping_section]
    }
}

pub(crate) fn update_server_stats(
    mut net: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut query: Query<&mut Text, With<ServerStats>>,
    mut state: ResMut<ServerStatsState>,
    fonts: Res<Fonts>,
    client: Res<RenetClient>,
) {
    for message in net.read() {
        let mut update = false;
        match &message.message {
            PlayerServerMessage::ConnectedPlayers(amount) => {
                state.connected_players = *amount;
                update = true;
                state.rtt = client.rtt() as f32;
            }
            _ => (),
        }

        if update {
            let mut text = query.get_single_mut().unwrap();
            text.sections = state.to_sections(fonts.handles.get(ARIZONE_FONT).unwrap().clone());
        }
    }
}
