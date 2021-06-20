use bevy::{core::Time, math::Vec3, prelude::{Entity, EventWriter, Query, Res, ResMut}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent};

use crate::space_core::{components::radio::Radio, enums::space_jobs::SpaceJobsEnum, events::net::net_chat_message::NetChatMessage, functions::new_chat_message::{Communicator, new_chat_message}, resources::{asana_boarding_announcements::AsanaBoardingAnnouncements, handle_to_entity::HandleToEntity}};

pub fn tick_asana_boarding_announcements(
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    handle_to_entity : Res<HandleToEntity>,
    radio_pawns : Query<(Entity, &Radio, &RigidBodyHandleComponent)>,
    rigid_bodies: Res<RigidBodySet>,
    mut asana_boarding_announcements : ResMut<AsanaBoardingAnnouncements>,
    time: Res<Time>,
) {

    let mut done_messages : Vec<String> = vec![];
    let mut vec_i = 0;

    for (announcement_message, announcement_timer) in &mut asana_boarding_announcements.announcements {

        if announcement_timer.tick(time.delta()).just_finished() {

            let sensed_by_vec : Vec<Entity> = vec![];

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
                None,
                &rigid_bodies
            );

            done_messages.insert(
                vec_i,
                announcement_message.to_string()
            );

            vec_i+=1;

        }

    }

    for done_message in done_messages {

        asana_boarding_announcements.announcements.remove(&done_message);

    }


}