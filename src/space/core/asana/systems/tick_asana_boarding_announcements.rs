use bevy_core::Time;
use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Query, Res, ResMut},
};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::core::{
    asana::resources::AsanaBoardingAnnouncements,
    chat::{
        components::Radio,
        events::NetChatMessage,
        functions::{new_chat_message, Communicator, MessagingPlayerState},
    },
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    pawn::components::{PersistentPlayerData, SpaceJobsEnum},
};

pub fn tick_asana_boarding_announcements(
    mut net_new_chat_message_event: EventWriter<NetChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    radio_pawns: Query<(
        Entity,
        &Radio,
        &RigidBodyPositionComponent,
        &PersistentPlayerData,
    )>,
    mut asana_boarding_announcements: ResMut<AsanaBoardingAnnouncements>,
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
                SpaceJobsEnum::Control,
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
