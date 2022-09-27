use api::gridmap::{to_doryen_coordinates, GridmapExamineMessages, GridmapMain};
use api::{
    chat::{
        EXAMINATION_EMPTY, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR,
    },
    data::HandleToEntity,
    examinable::Examinable,
    gridmap::{GridMapLayer, GridmapDetails1},
    health::{HealthComponent, HealthContainer},
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    sensable::Sensable,
    senser::Senser,
};
use bevy::prelude::{warn, Query, Res, ResMut};
use networking::messages::ExamineEntityMessages;

pub(crate) struct ExamineEntityPawn {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for ExamineEntityPawn {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Examine an entity's health.
pub(crate) fn examine_entity(
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

pub(crate) struct NetConnExamine {
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

/// Examine a ship cell's health.
pub(crate) fn examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    senser_entities: Query<&Senser>,
) {
    for examine_event in examine_map_events.messages.iter_mut() {
        let examiner_senser_component;

        match senser_entities.get(examine_event.entity) {
            Ok(examiner_senser) => {
                examiner_senser_component = examiner_senser;
            }
            Err(_rr) => {
                warn!("Couldn't find examiner entity in &Senser query.");
                continue;
            }
        }

        let gridmap_result;

        match examine_event.gridmap_type {
            GridMapLayer::Main => {
                gridmap_result = gridmap_main.grid_data.get(&examine_event.gridmap_cell_id);
            }
            GridMapLayer::Details1 => {
                gridmap_result = gridmap_details1
                    .grid_data
                    .get(&examine_event.gridmap_cell_id);
            }
        }

        let ship_cell_option;

        match gridmap_result {
            Some(gridmap_cell) => ship_cell_option = Some(gridmap_cell),
            None => {
                ship_cell_option = None;
            }
        }

        let coords = to_doryen_coordinates(
            examine_event.gridmap_cell_id.x,
            examine_event.gridmap_cell_id.z,
        );
        if examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            match ship_cell_option {
                Some(ship_cell) => {
                    let mut message = "".to_string();

                    let structure_health;

                    match &ship_cell.health.health_container {
                        HealthContainer::Structure(t) => {
                            structure_health = t;
                        }
                        _ => {
                            continue;
                        }
                    }

                    if structure_health.brute < 25.
                        && structure_health.burn < 25.
                        && structure_health.toxin < 25.
                    {
                        message = message
                            + "[font="
                            + FURTHER_ITALIC_FONT
                            + "][color="
                            + HEALTHY_COLOR
                            + "]\nIt is in perfect shape.[/color][/font]";
                    } else {
                        if structure_health.brute > 75. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt is heavily damaged.[/color][/font]";
                        } else if structure_health.brute > 50. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt is damaged.[/color][/font]";
                        } else if structure_health.brute > 25. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt is slightly damaged.[/color][/font]";
                        }

                        if structure_health.burn > 75. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt suffers from heavy burn damage.[/color][/font]";
                        } else if structure_health.burn > 50. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt suffers burn damage.[/color][/font]";
                        } else if structure_health.burn > 25. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt has slight burn damage.[/color][/font]";
                        }
                    }
                    examine_event.message = examine_event.message.clone() + &message + "\n";
                }
                None => {}
            }
        }
    }
}
