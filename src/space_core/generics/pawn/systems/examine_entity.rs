use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, warn};

use crate::space_core::{entities::human_male_pawn::spawn::generate_human_examine_text, generics::{pawn::{components::{Senser, StandardCharacter}, events::{InputExamineEntity, NetExamineEntity}, resources::HandleToEntity, functions::new_chat_message::{FURTHER_NORMAL_FONT, ASTRIX, UNHEALTHY_COLOR, HEALTHY_COLOR, FURTHER_ITALIC_FONT}}, inventory::components::Inventory, health::components::Health, entity::components::{Examinable, Sensable}, networking::resources::ReliableServerMessage, gridmap::functions::examine_cell::EXAMINATION_EMPTY}};

pub fn examine_entity(
    mut examine_entity_events : EventReader<InputExamineEntity>,
    mut net_new_chat_message_event : EventWriter<NetExamineEntity>,
    handle_to_entity : Res<HandleToEntity>,
    criteria_query : Query<&Senser>,
    q0 : Query<(&Examinable, &Sensable, &Health)>,
    q1 : Query<(&Examinable, &Sensable, &Health, &Inventory, &StandardCharacter)>,
    q2 : Query<&Examinable>,
) {

    for examine_event in examine_entity_events.iter() {

        let entity_reference = Entity::from_bits(examine_event.examine_entity_bits);

        // Safety check.
        match criteria_query.get(examine_event.entity) {
            Ok(_) => {},
            Err(_rr) => {
                continue;
            },
        }

        match q1.get(entity_reference) {
            Ok((_examinable_component, _sensable_component, health_component, inventory_component, standard_character_component)) => {

                let text = generate_human_examine_text(
                    &standard_character_component.character_name,
                    Some(inventory_component),
                    &q2,
                    health_component,
                );

                net_new_chat_message_event.send(NetExamineEntity {
                    handle: examine_event.handle,
                    message: ReliableServerMessage::ChatMessage(text),
                });

                continue;

            },
            Err(_rr) => {},
        }

        match q0.get(entity_reference) {
            Ok((examinable_component, sensable_component, health_component)) => {
                //found=true;
                let mut text = "".to_string();

                match &health_component.health_container {
                    crate::space_core::generics::health::components::HealthContainer::Entity(entity_container) => {
        
                        let mut examinable_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
                        for (_text_id, assigned_text) in examinable_component.assigned_texts.iter() {
                            examinable_text = examinable_text + assigned_text;
                            examinable_text = examinable_text + "\n";
                        }
        
                        if entity_container.brute < 25. && entity_container.burn < 25. && entity_container.toxin < 25. {
        
                            examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + HEALTHY_COLOR + "]It is in perfect shape.[/color][/font]";
        
                        } else {
        
                            if entity_container.brute > 75. {
                                examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]It is heavily damaged.[/color][/font]";
                            } else if entity_container.brute > 50. {
                                examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]It is damaged.[/color][/font]";
                            } else if entity_container.brute > 25. {
                                examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]It is slightly damaged.[/color][/font]";
                            }

                            if entity_container.burn > 75. {
                                examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt suffers from heavy burn damage.[/color][/font]";
                            } else if entity_container.burn > 50. {
                                examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt suffers burn damage.[/color][/font]";
                            } else if entity_container.burn > 25. {
                                examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt has slight burn damage.[/color][/font]";
                            }
        
                        }
                        
                        examinable_text = examinable_text + "\n" + ASTRIX + "[/font]";
                        
                        text = examinable_text;

                    },
                    _=>(),
                }

                let entity = handle_to_entity.map.get(&examine_event.handle)
                .expect("examine_entity.rs could not find the entity belonging to examining handle.");


                if !sensable_component.sensed_by.contains(entity) {

                    text = EXAMINATION_EMPTY.to_string();

                }

                net_new_chat_message_event.send(NetExamineEntity {
                    handle: examine_event.handle,
                    message: ReliableServerMessage::ChatMessage(text),
                });

            },
            Err(_rr) => {
                warn!("Couldn't find user input requested examinable entity.");
            },
        }

    

        

    }

}
