use bevy::{
    prelude::{EventWriter, Res, ResMut},
    time::Time,
};
use chat::chat::NewChatMessage;

use player::boarding::BoardingAnnouncements;
/// Manage asana boarding announcements.

pub(crate) fn tick_asana_boarding_announcements(
    mut net_new_chat_message_event: EventWriter<NewChatMessage>,
    mut asana_boarding_announcements: ResMut<BoardingAnnouncements>,
    time: Res<Time>,
) {
    let mut done_messages: Vec<usize> = vec![];

    let mut j = 0;

    for (announcement_message, announcement_timer) in
        &mut asana_boarding_announcements.announcements
    {
        if announcement_timer.tick(time.delta()).just_finished() {
            net_new_chat_message_event.send(NewChatMessage {
                messenger_entity_option: None,
                messenger_name_option: Some("ASANA".to_string()),
                raw_message: announcement_message.to_string(),
                exclusive_radio: true,
                position_option: None,
                send_entity_update: false,
            });

            done_messages.push(j);
        }

        j += 1;
    }

    for j in done_messages {
        asana_boarding_announcements.announcements.remove(j);
    }
}
