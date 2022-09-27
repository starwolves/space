use api::{
    chat::END_ASTRIX,
    gridmap::GridmapExamineMessages,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};
use bevy::prelude::{EventWriter, ResMut};

use api::{
    chat::{ASTRIX, EXAMINATION_EMPTY, FURTHER_NORMAL_FONT},
    data::HandleToEntity,
    examinable::Examinable,
    health::{HealthComponent, HealthContainer},
    sensable::Sensable,
    senser::Senser,
};
use bevy::prelude::{warn, Query, Res};

use networking::messages::ExamineEntityMessages;

pub(crate) struct NetExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetExamine {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Finalize examining the ship gridmap.
pub(crate) fn finalize_examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    mut net: EventWriter<NetExamine>,
) {
    for event in examine_map_events.messages.iter_mut() {
        event.message = event.message.to_string() + END_ASTRIX;

        net.send(NetExamine {
            handle: event.handle,
            message: ReliableServerMessage::ChatMessage(event.message.clone()),
        });
    }

    examine_map_events.messages.clear();
}

pub struct NetConnExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetConnExamine {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Manage examining an entity.
pub fn examine_entity(
    mut examine_entity_events: ResMut<ExamineEntityMessages>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&Senser>,
    q0: Query<(&Examinable, &Sensable, &HealthComponent)>,
) {
    for examine_event in examine_entity_events.messages.iter_mut() {
        let entity_reference = examine_event.examine_entity;

        // Safety check.
        match criteria_query.get(examine_event.entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        match q0.get(entity_reference) {
            Ok((examinable_component, sensable_component, health_component)) => {
                let mut text = "".to_string();

                match &health_component.health.health_container {
                    HealthContainer::Entity(_entity_container) => {
                        let mut examinable_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]";
                        for (_text_id, assigned_text) in examinable_component.assigned_texts.iter()
                        {
                            examinable_text = examinable_text + "\n";
                            examinable_text = examinable_text + assigned_text;
                        }

                        examinable_text = examinable_text + "\n" + "[/font]";

                        if examinable_component.assigned_texts.len() > 0 {
                            text = examinable_text;
                        }
                    }
                    _ => (),
                }

                let entity = handle_to_entity.map.get(&examine_event.handle).expect(
                    "examine_entity.rs could not find the entity belonging to examining handle.",
                );

                if !sensable_component.sensed_by.contains(entity) {
                    text = EXAMINATION_EMPTY.to_string();
                }

                examine_event.message = examine_event.message.clone() + &text;
            }
            Err(_rr) => {
                warn!("Couldn't find user input requested examinable entity.");
            }
        }
    }
}

/// Finalize examining an entity.
pub fn finalize_examine_entity(
    mut examine_map_events: ResMut<ExamineEntityMessages>,
    mut net: EventWriter<NetConnExamine>,
) {
    for event in examine_map_events.messages.iter_mut() {
        event.message = event.message.to_string() + "\n" + ASTRIX;

        net.send(NetConnExamine {
            handle: event.handle,
            message: ReliableServerMessage::ChatMessage(event.message.clone()),
        });
    }

    examine_map_events.messages.clear();
}
