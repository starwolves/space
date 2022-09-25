use api::data::{ConnectedPlayer, HandleToEntity};
use bevy::{
    math::Vec3,
    prelude::{Entity, EventWriter, Query, Res, ResMut, Transform},
    time::Time,
};
use chat::chat::{new_chat_message, Communicator, MessagingPlayerState, NetChatMessage, Radio};
use pawn::pawn::{PersistentPlayerData, ShipJobsEnum};
use player_controller::boarding::BoardingAnnouncements;

/// Manage asana boarding announcements.
pub(crate) fn tick_asana_boarding_announcements(
    mut net_new_chat_message_event: EventWriter<NetChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    radio_pawns: Query<(Entity, &Radio, &Transform, &PersistentPlayerData)>,
    mut asana_boarding_announcements: ResMut<BoardingAnnouncements>,
    time: Res<Time>,
    global_listeners: Query<(&ConnectedPlayer, &PersistentPlayerData)>,
) {
    let mut done_messages: Vec<usize> = vec![];

    let mut j = 0;

    for (announcement_message, announcement_timer) in
        &mut asana_boarding_announcements.announcements
    {
        if announcement_timer.tick(time.delta()).just_finished() {
            let sensed_by_vec: Vec<Entity> = vec![];

            new_chat_message(
                &mut net_new_chat_message_event,
                &handle_to_entity,
                &sensed_by_vec,
                &sensed_by_vec,
                Vec3::ZERO,
                "ASANA".to_string(),
                ShipJobsEnum::Control,
                announcement_message.to_string(),
                Communicator::Machine,
                true,
                &radio_pawns,
                &global_listeners,
                None,
                None,
                &MessagingPlayerState::Alive,
            );

            done_messages.push(j);
        }

        j += 1;
    }

    for j in done_messages {
        asana_boarding_announcements.announcements.remove(j);
    }
}
