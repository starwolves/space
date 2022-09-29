use std::collections::BTreeMap;

use api::chat::END_ASTRIX;
use bevy::prelude::{Component, EventReader, EventWriter, ResMut, SystemLabel};

use api::chat::{ASTRIX, EXAMINATION_EMPTY, FURTHER_NORMAL_FONT};
use bevy::prelude::{warn, Query, Res};
use networking::messages::InputExamineEntity;
use networking::messages::PendingMessage;
use networking::messages::PendingNetworkMessage;
use networking::messages::{InputExamineMap, ReliableServerMessage};
use networking_macros::NetMessage;
use sensable::core::Sensable;
use senser::senser::Senser;
use server::core::HandleToEntity;

#[derive(NetMessage)]
pub(crate) struct NetExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
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
#[derive(NetMessage)]
pub struct NetConnExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// Manage examining an entity.
pub fn examine_entity(
    mut examine_entity_events: ResMut<ExamineEntityMessages>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&Senser>,
    q0: Query<(&Examinable, &Sensable)>,
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
            Ok((examinable_component, sensable_component)) => {
                let mut text = "".to_string();

                let mut examinable_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]";
                for (_text_id, assigned_text) in examinable_component.assigned_texts.iter() {
                    examinable_text = examinable_text + "\n";
                    examinable_text = examinable_text + assigned_text;
                }

                examinable_text = examinable_text + "\n" + "[/font]";

                if examinable_component.assigned_texts.len() > 0 {
                    text = examinable_text;
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

/// Component for entities that can be examined.
#[derive(Component, Default)]
pub struct Examinable {
    pub assigned_texts: BTreeMap<u32, String>,
    pub name: RichName,
}

/// A rich examinable name for an entity.
#[derive(Clone, Debug)]
pub struct RichName {
    pub name: String,
    pub n: bool,
    pub the: bool,
}

impl RichName {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_a_name(&self) -> String {
        let prefix;
        if self.the {
            prefix = "the";
        } else {
            if self.n {
                prefix = "an";
            } else {
                prefix = "a";
            }
        }
        prefix.to_owned() + " " + &self.name
    }
}

impl Default for RichName {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            n: false,
            the: false,
        }
    }
}

/// System label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ExamineLabels {
    Start,
    Default,
}

/// Stores examine messages being built this frame for gridmap examination.
#[derive(Default)]
pub struct GridmapExamineMessages {
    pub messages: Vec<InputExamineMap>,
}
/// Resource with client inputs of examining entity messages.
#[derive(Default)]
pub struct ExamineEntityMessages {
    pub messages: Vec<InputExamineEntity>,
}

pub fn finalize_entity_examine_input(
    mut examine_messages: ResMut<ExamineEntityMessages>,
    mut entity_examine_input: EventReader<InputExamineEntity>,
) {
    for input_event in entity_examine_input.iter() {
        examine_messages.messages.push(input_event.clone());
    }
}
