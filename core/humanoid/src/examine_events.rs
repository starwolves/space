use bevy::prelude::{Query, ResMut};
use entity::examine::Examinable;
use entity::examine::ExamineEntityMessages;
use entity::health::HealthComponent;
use entity::sensable::Sensable;
use entity::senser::Senser;
use inventory::inventory::Inventory;
use pawn::pawn::Pawn;
use text_api::core::FURTHER_NORMAL_FONT;

/// Examine a humanoid entity.

pub(crate) fn examine_entity(
    mut examine_entity_events: ResMut<ExamineEntityMessages>,
    criteria_query: Query<&Senser>,
    q1: Query<(&Examinable, &Sensable, &HealthComponent, &Inventory, &Pawn)>,
    _q2: Query<&Examinable>,
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

        match q1.get(entity_reference) {
            Ok((
                _examinable_component,
                _sensable_component,
                _health_component,
                _inventory_component,
                standard_character_component,
            )) => {
                let mut text = "[font=".to_owned()
                    + FURTHER_NORMAL_FONT
                    + "]"
                    + "\n"
                    + &standard_character_component.character_name
                    + ", a Security Officer.\n"
                    + "He is human.\n";

                let mut examine_text = "".to_string();

                examine_text = examine_text + "\n";
                /*
                for slot in inventory_component.slots.iter() {
                    match slot.space {
                        Some(_slot_item_entity) => {
                            let examinable = q2.get(slot_item_entity)
                                        .expect("inventory_update.rs::generate_human_examine_text couldn't find inventory_item_component of an item from passed inventory.");

                            if slot.id == "left_hand" {
                                examine_text = examine_text
                                    + "He is holding "
                                    + &examinable.name.get_a_name()
                                    + " in his left hand.\n";
                            } else if slot.id == "right_hand" {
                                examine_text = examine_text
                                    + "He is holding "
                                    + &examinable.name.get_a_name()
                                    + " in his right hand.\n";
                            } else if slot.id == "helmet" {
                                examine_text = examine_text
                                    + "He is wearing "
                                    + &examinable.name.get_a_name()
                                    + " on his head.\n";
                            } else if slot.id == "jumpsuit" {
                                examine_text = examine_text
                                    + "He is wearing "
                                    + &examinable.name.get_a_name()
                                    + " on his body.\n";
                            } else if slot.id == "holster" {
                                examine_text = examine_text
                                    + &examinable.name.get_a_name()
                                    + " is attached to his holster.\n";
                            } else {
                                examine_text = examine_text
                                    + "He is wearing "
                                    + &examinable.name.get_a_name()
                                    + ".\n";
                            }
                        }
                        None => {}
                    }
                }*/

                text = text + &examine_text;

                examine_event.message = examine_event.message.clone() + &text;
            }
            Err(_rr) => {}
        }
    }
}
