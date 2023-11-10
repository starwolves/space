use bevy::{
    prelude::{Color, EventWriter, Query, Res, ResMut},
    time::{Fixed, Time},
};

use chat::net::{ChatMessage, ChatServerMessage};
use networking::server::{ConnectedPlayer, OutgoingReliableServerMessage};
use player::boarding::BoardingAnnouncements;
use ui::{
    fonts::{Fonts, SOURCECODE_REGULAR_FONT},
    text::{NetTextSection, COMMUNICATION_FONT_SIZE},
};
/// Manage asana boarding announcements.

pub(crate) fn tick_asana_boarding_announcements(
    mut asana_boarding_announcements: ResMut<BoardingAnnouncements>,
    mut net: EventWriter<OutgoingReliableServerMessage<ChatServerMessage>>,
    connected_players: Query<&ConnectedPlayer>,
    fonts: Res<Fonts>,
    fixed_time: Res<Time<Fixed>>,
) {
    let mut done_messages: Vec<usize> = vec![];

    let mut j = 0;

    for (announcement_message, announcement_timer) in
        &mut asana_boarding_announcements.announcements
    {
        if announcement_timer.tick(fixed_time.delta()).just_finished() {
            for player in connected_players.iter() {
                if !player.connected {
                    continue;
                }

                net.send(OutgoingReliableServerMessage {
                    handle: player.handle,
                    message: ChatServerMessage::ChatMessage(ChatMessage {
                        sections: vec![NetTextSection {
                            text: "ASANA: ".to_string() + announcement_message,
                            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
                            font_size: COMMUNICATION_FONT_SIZE,
                            color: Color::WHITE,
                        }],
                    }),
                });
            }

            done_messages.push(j);
        }

        j += 1;
    }

    for j in done_messages {
        asana_boarding_announcements.announcements.remove(j);
    }
}
