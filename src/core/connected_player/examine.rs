use bevy::prelude::{warn, Entity, EventReader, EventWriter, Query, Res};

use crate::{
    core::{
        atmospherics::difussion::{get_atmos_index, AtmosphericsResource, CELCIUS_KELVIN_OFFSET},
        chat::{
            message::{
                ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR,
            },
            net::NetChatMessage,
        },
        examinable::examinable::Examinable,
        gridmap::gridmap::{
            examine_ship_cell, get_empty_cell_message, get_space_message, to_doryen_coordinates,
            GridmapData, GridmapDetails1, GridmapMain, Vec2Int, Vec3Int, END_ASTRIX,
            EXAMINATION_EMPTY,
        },
        health::health::{Health, HealthContainer},
        humanoid::humanoid::Humanoid,
        inventory::inventory::Inventory,
        networking::networking::{GridMapType, ReliableServerMessage},
        sensable::sensable::Sensable,
        senser::visible_checker::{Senser, SensingAbility},
    },
    entities::human_male::examine::generate_human_examine_text,
};

use super::{net::NetExamineEntity, plugin::HandleToEntity};

pub fn examine_entity(
    mut examine_entity_events: EventReader<InputExamineEntity>,
    mut net_new_chat_message_event: EventWriter<NetExamineEntity>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&Senser>,
    q0: Query<(&Examinable, &Sensable, &Health)>,
    q1: Query<(&Examinable, &Sensable, &Health, &Inventory, &Humanoid)>,
    q2: Query<&Examinable>,
) {
    for examine_event in examine_entity_events.iter() {
        let entity_reference = Entity::from_bits(examine_event.examine_entity_bits);

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
                health_component,
                inventory_component,
                standard_character_component,
            )) => {
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
            }
            Err(_rr) => {}
        }

        match q0.get(entity_reference) {
            Ok((examinable_component, sensable_component, health_component)) => {
                //found=true;
                let mut text = "".to_string();

                match &health_component.health_container {
                    HealthContainer::Entity(entity_container) => {
                        let mut examinable_text =
                            "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
                        for (_text_id, assigned_text) in examinable_component.assigned_texts.iter()
                        {
                            examinable_text = examinable_text + assigned_text;
                            examinable_text = examinable_text + "\n";
                        }

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

                        examinable_text = examinable_text + "\n" + ASTRIX + "[/font]";

                        text = examinable_text;
                    }
                    _ => (),
                }

                let entity = handle_to_entity.map.get(&examine_event.handle).expect(
                    "examine_entity.rs could not find the entity belonging to examining handle.",
                );

                if !sensable_component.sensed_by.contains(entity) {
                    text = EXAMINATION_EMPTY.to_string();
                }

                net_new_chat_message_event.send(NetExamineEntity {
                    handle: examine_event.handle,
                    message: ReliableServerMessage::ChatMessage(text),
                });
            }
            Err(_rr) => {
                warn!("Couldn't find user input requested examinable entity.");
            }
        }
    }
}

pub struct InputExamineEntity {
    pub handle: u64,
    pub examine_entity_bits: u64,
    pub entity: Entity,
}

pub struct InputExamineMap {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub fn examine_map(
    mut examine_map_events: EventReader<InputExamineMap>,
    mut net_new_chat_message_event: EventWriter<NetChatMessage>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    senser_entities: Query<&Senser>,
    gridmap_data: Res<GridmapData>,
    atmospherics_resource: Res<AtmosphericsResource>,
) {
    for examine_event in examine_map_events.iter() {
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

        let mut examine_text;

        let coords = to_doryen_coordinates(
            examine_event.gridmap_cell_id.x,
            examine_event.gridmap_cell_id.z,
        );
        if !examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            examine_text = get_empty_cell_message();
        } else {
            let gridmap_type = &examine_event.gridmap_type;

            let gridmap_result;

            match examine_event.gridmap_type {
                GridMapType::Main => {
                    gridmap_result = gridmap_main.grid_data.get(&examine_event.gridmap_cell_id);
                }
                GridMapType::Details1 => {
                    gridmap_result = gridmap_details1.data.get(&examine_event.gridmap_cell_id);
                }
            }

            let ship_cell_option;

            match gridmap_result {
                Some(gridmap_cell) => ship_cell_option = Some(gridmap_cell),
                None => {
                    ship_cell_option = None;
                }
            }

            match ship_cell_option {
                Some(ship_cell) => {
                    examine_text = examine_ship_cell(ship_cell, gridmap_type, &gridmap_data);
                }
                None => {
                    examine_text = get_space_message();
                }
            }
        }

        examine_text = examine_text + "\n";

        for sensing_ability in examiner_senser_component.sensing_abilities.iter() {
            match sensing_ability {
                SensingAbility::AtmosphericsSensor => {
                    let id = Vec2Int {
                        x: examine_event.gridmap_cell_id.x,
                        y: examine_event.gridmap_cell_id.z,
                    };

                    if AtmosphericsResource::is_id_out_of_range(id) {
                        continue;
                    }

                    let atmospherics = atmospherics_resource
                        .atmospherics
                        .get(get_atmos_index(id))
                        .unwrap();

                    if atmospherics.blocked {
                        continue;
                    }

                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + ATMOSPHERICS_TEXT_COLOR
                        + "]"
                        + "\n"
                        + "Atmospherics DataLink: [/color]"
                        + "\n"
                        + "Temperature: "
                        + &(atmospherics.temperature - CELCIUS_KELVIN_OFFSET)
                            .floor()
                            .to_string()
                        + " c\n"
                        + "Pressure: "
                        + &atmospherics.get_pressure().floor().to_string()
                        + " kpa"
                        + "[/font]";
                }
                SensingAbility::ShipEngineerKnowledge => {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + ENGINEERING_TEXT_COLOR
                        + "]"
                        + "\n"
                        + "Ship Engineer Knowledge: [/color]"
                        + "\n"
                        + "Reference shows coordinates ("
                        + &examine_event.gridmap_cell_id.x.to_string()
                        + " , "
                        + &examine_event.gridmap_cell_id.z.to_string()
                        + ")."
                        + "[/font]";
                }
            }
        }

        examine_text = examine_text + END_ASTRIX;

        net_new_chat_message_event.send(NetChatMessage {
            handle: examine_event.handle,
            message: ReliableServerMessage::ChatMessage(examine_text),
        });
    }
}

pub const ATMOSPHERICS_TEXT_COLOR: &str = "#1797ff";
pub const ENGINEERING_TEXT_COLOR: &str = "#ff992b";
