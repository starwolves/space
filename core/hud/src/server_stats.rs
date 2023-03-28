use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, EventReader, Handle, NodeBundle, Query, Res,
        ResMut, Resource, TextBundle, With,
    },
    text::{Font, Text, TextSection, TextStyle},
    time::{Time, Timer, TimerMode},
    ui::{Size, Style, Val},
};
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
                        size: Size::new(Val::Percent(5.), Val::Percent(21.)),
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

#[derive(Resource)]
pub struct ServerStatsState {
    pub server_timer: Timer,
    pub connected_players: u16,
    pub ping: u16,
}

impl Default for ServerStatsState {
    fn default() -> Self {
        Self {
            server_timer: Timer::from_seconds(2., TimerMode::Repeating),
            connected_players: 0,
            ping: 0,
        }
    }
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
            value: format!("{} ms.", self.ping),
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
    time: Res<Time>,
    fonts: Res<Fonts>,
) {
    for message in net.iter() {
        let mut update = false;
        match &message.message {
            PlayerServerMessage::ServerTime => {
                state.server_timer.tick(time.delta());
                let mut substracted = 2000;
                let elapsed = state.server_timer.elapsed().as_millis();
                if elapsed <= 2000 {
                    substracted = elapsed;
                }
                let ping = (elapsed - substracted) as u16;
                state.ping = ping;
                update = true;
                state.server_timer.reset();
            }
            PlayerServerMessage::ConnectedPlayers(amount) => {
                state.connected_players = *amount;
                update = true;
            }
            _ => (),
        }

        if update {
            let mut text = query.get_single_mut().unwrap();
            text.sections = state.to_sections(fonts.handles.get(ARIZONE_FONT).unwrap().clone());
        }
    }
}
