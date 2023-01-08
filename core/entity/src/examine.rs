use crate::health::HealthComponent;
use crate::health::HealthContainer;
use crate::sensable::Sensable;
use crate::senser::Senser;
use std::collections::BTreeMap;

use bevy::prelude::Entity;
use bevy::prelude::Resource;
use bevy::prelude::{warn, Query, Res};
use bevy::prelude::{Component, EventReader, ResMut, SystemLabel};
use text_api::core::FURTHER_ITALIC_FONT;
use text_api::core::HEALTHY_COLOR;
use text_api::core::UNHEALTHY_COLOR;
use text_api::core::{ASTRIX, EXAMINATION_EMPTY, FURTHER_NORMAL_FONT};

use networking::server::HandleToEntity;

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
use networking::server::NetworkingChatServerMessage;
use networking::server::OutgoingReliableServerMessage;

use bevy::prelude::EventWriter;
/// Finalize examining an entity.

pub(crate) fn finalize_examine_entity(
    mut examine_map_events: ResMut<ExamineEntityMessages>,
    mut net: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
) {
    for event in examine_map_events.messages.iter_mut() {
        event.message = event.message.to_string() + "\n" + ASTRIX;

        net.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: NetworkingChatServerMessage::ChatMessage(event.message.clone()),
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

/// Resource with client inputs of examining entity messages.
#[derive(Default, Resource)]

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

/// Examine an entity's health.

pub(crate) fn examine_entity_health(
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
            Ok((_examinable_component, sensable_component, health_component)) => {
                //found=true;
                let mut text;

                match &health_component.health.health_container {
                    HealthContainer::Entity(entity_container) => {
                        let mut examinable_text =
                            "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + "\n";

                        if entity_container.brute < 25.
                            && entity_container.burn < 25.
                            && entity_container.toxin < 25.
                        {
                            examinable_text = examinable_text
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + HEALTHY_COLOR
                                + "]It is in perfect shape.[/color][/font]";
                        } else {
                            if entity_container.brute > 75. {
                                examinable_text = examinable_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]It is heavily damaged.[/color][/font]";
                            } else if entity_container.brute > 50. {
                                examinable_text = examinable_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]It is damaged.[/color][/font]";
                            } else if entity_container.brute > 25. {
                                examinable_text = examinable_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]It is slightly damaged.[/color][/font]";
                            }

                            if entity_container.burn > 75. {
                                examinable_text = examinable_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]\nIt suffers from heavy burn damage.[/color][/font]";
                            } else if entity_container.burn > 50. {
                                examinable_text = examinable_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]\nIt suffers burn damage.[/color][/font]";
                            } else if entity_container.burn > 25. {
                                examinable_text = examinable_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]\nIt has slight burn damage.[/color][/font]";
                            }
                        }

                        examinable_text = examinable_text + "[/font]\n";

                        text = examinable_text;
                    }
                    HealthContainer::Humanoid(humanoid_container) => {
                        let mut examine_text = "".to_string();
                        let head_damage = humanoid_container.head_brute
                            + humanoid_container.head_burn
                            + humanoid_container.head_toxin;
                        let torso_damage = humanoid_container.torso_brute
                            + humanoid_container.torso_burn
                            + humanoid_container.torso_toxin;
                        let left_arm_damage = humanoid_container.left_arm_brute
                            + humanoid_container.left_arm_burn
                            + humanoid_container.left_arm_toxin;
                        let right_arm_damage = humanoid_container.right_arm_brute
                            + humanoid_container.right_arm_burn
                            + humanoid_container.right_arm_toxin;
                        let left_leg_damage = humanoid_container.left_leg_brute
                            + humanoid_container.left_leg_burn
                            + humanoid_container.left_leg_toxin;
                        let right_leg_damage = humanoid_container.right_leg_brute
                            + humanoid_container.right_leg_burn
                            + humanoid_container.right_leg_toxin;

                        if head_damage < 25.
                            && torso_damage < 25.
                            && left_arm_damage < 25.
                            && right_arm_damage < 25.
                            && left_leg_damage < 25.
                            && right_leg_damage < 25.
                        {
                            examine_text = examine_text
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + HEALTHY_COLOR
                                + "]He is in perfect shape.[/color][/font]\n";
                        } else {
                            if humanoid_container.head_brute > 75. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His head is heavily injured.[/color][/font]\n";
                            } else if humanoid_container.head_brute > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His head is injured.[/color][/font]\n";
                            } else if humanoid_container.head_brute > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His head is bruised.[/color][/font]\n";
                            }

                            if humanoid_container.torso_brute > 75. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His torso is heavily injured.[/color][/font]\n";
                            } else if humanoid_container.torso_brute > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His torso is injured.[/color][/font]\n";
                            } else if humanoid_container.torso_brute > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His torso is bruised.[/color][/font]\n";
                            }

                            if humanoid_container.left_arm_brute > 75. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His left arm is heavily injured.[/color][/font]\n";
                            } else if humanoid_container.left_arm_brute > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His left arm is injured.[/color][/font]\n";
                            } else if humanoid_container.left_arm_brute > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His left arm is bruised.[/color][/font]\n";
                            }

                            if humanoid_container.right_arm_brute > 75. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His right arm is heavily injured.[/color][/font]\n";
                            } else if humanoid_container.right_arm_brute > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His right arm is injured.[/color][/font]\n";
                            } else if humanoid_container.right_arm_brute > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His right arm is bruised.[/color][/font]\n";
                            }

                            if humanoid_container.left_leg_brute > 75. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His left leg is heavily injured.[/color][/font]\n";
                            } else if humanoid_container.left_leg_brute > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His left leg is injured.[/color][/font]\n";
                            } else if humanoid_container.left_leg_brute > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His left leg is bruised.[/color][/font]\n";
                            }

                            if humanoid_container.right_leg_brute > 75. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His right leg is heavily injured.[/color][/font]\n";
                            } else if humanoid_container.right_leg_brute > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His right leg is injured.[/color][/font]\n";
                            } else if humanoid_container.right_leg_brute > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His right leg is bruised.[/color][/font]\n";
                            }

                            if humanoid_container.head_burn > 75. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His head has visible third degree burns, ouch![/color][/font]\n";
                            } else if humanoid_container.head_burn > 50. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His head has visible second degree burns.[/color][/font]\n";
                            } else if humanoid_container.head_burn > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His head has visible first degree burns.[/color][/font]\n";
                            }

                            if humanoid_container.torso_burn > 75. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His torso has visible third degree burns, ouch![/color][/font]\n";
                            } else if humanoid_container.torso_burn > 50. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His torso has visible second degree burns.[/color][/font]\n";
                            } else if humanoid_container.torso_burn > 25. {
                                examine_text = examine_text
                                    + "[font="
                                    + FURTHER_ITALIC_FONT
                                    + "][color="
                                    + UNHEALTHY_COLOR
                                    + "]His torso has visible first degree burns.[/color][/font]\n";
                            }

                            if humanoid_container.left_arm_burn > 75. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His left arm has visible third degree burns, ouch![/color][/font]\n";
                            } else if humanoid_container.left_arm_burn > 50. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His left arm has visible second degree burns.[/color][/font]\n";
                            } else if humanoid_container.left_arm_burn > 25. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His left arm has visible first degree burns.[/color][/font]\n";
                            }

                            if humanoid_container.right_arm_burn > 75. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His right arm has visible third degree burns, ouch![/color][/font]\n";
                            } else if humanoid_container.right_arm_burn > 50. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His right arm has visible second degree burns.[/color][/font]\n";
                            } else if humanoid_container.right_arm_burn > 25. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His right arm has visible first degree burns.[/color][/font]\n";
                            }

                            if humanoid_container.left_leg_burn > 75. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His left leg has visible third degree burns, ouch![/color][/font]\n";
                            } else if humanoid_container.left_leg_burn > 50. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His left leg has visible second degree burns.[/color][/font]\n";
                            } else if humanoid_container.left_leg_burn > 25. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His left leg has visible first degree burns.[/color][/font]\n";
                            }

                            if humanoid_container.right_leg_burn > 75. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His right leg has visible third degree burns, ouch![/color][/font]\n";
                            } else if humanoid_container.right_leg_burn > 50. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His right leg has visible second degree burns.[/color][/font]\n";
                            } else if humanoid_container.right_leg_burn > 25. {
                                examine_text = examine_text
                                            + "[font="
                                            + FURTHER_ITALIC_FONT
                                            + "][color="
                                            + UNHEALTHY_COLOR
                                            + "]His right leg has visible first degree burns.[/color][/font]\n";
                            }
                        }
                        text = "\n".to_string() + &examine_text;
                    }
                    HealthContainer::Structure(_) => {
                        continue;
                    }
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

/// Input examine entity event.
#[derive(Clone)]

pub struct InputExamineEntity {
    pub handle: u64,
    pub examine_entity: Entity,
    pub entity: Entity,
    /// Examine message that is being built and returned to the client.
    pub message: String,
}

impl Default for InputExamineEntity {
    fn default() -> Self {
        Self {
            handle: 0,
            examine_entity: Entity::from_bits(0),
            entity: Entity::from_bits(0),
            message: ASTRIX.to_string(),
        }
    }
}

use networking::server::IncomingReliableClientMessage;

use crate::net::EntityClientMessage;
/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<EntityClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_entity: EventWriter<InputExamineEntity>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            EntityClientMessage::ExamineEntity(entity_id) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_examine_entity.send(InputExamineEntity {
                            handle: message.handle,
                            examine_entity: Entity::from_bits(entity_id),
                            entity: *player_entity,
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to ExamineEntity sender handle."
                        );
                    }
                }
            }
        }
    }
}
