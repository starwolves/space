use bevy::prelude::{EventReader, EventWriter};

use crate::space_core::events::{general::examine_map::ExamineMap, net::net_chat_message::NetChatMessage};

pub fn examine_map(
    mut examine_map_events : EventReader<ExamineMap>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
) {

    for examine_event in examine_map_events.iter() {

        

    }

}
