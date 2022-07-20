use bevy::prelude::{warn, Entity, EventWriter, Query, Res, ResMut};
use const_format::concatcp;
use shared::{
    chat::{
        get_empty_cell_message, get_space_message, ASTRIX, EXAMINATION_EMPTY, FURTHER_NORMAL_FONT,
    },
    data::HandleToEntity,
    examinable::Examinable,
    gridmap::{
        to_doryen_coordinates, GridMapType, GridmapData, GridmapDetails1, GridmapExamineMessages,
        GridmapMain,
    },
    health::{Health, HealthContainer},
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    sensable::Sensable,
    senser::Senser,
};

use gridmap::events::examine_ship_cell;
use networking::messages::ExamineEntityMessages;

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

pub fn examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    senser_entities: Query<&Senser>,
    gridmap_data: Res<GridmapData>,
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

        examine_event.message = examine_event.message.clone() + &examine_text;
    }
}

pub fn examine_entity(
    mut examine_entity_events: ResMut<ExamineEntityMessages>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&Senser>,
    q0: Query<(&Examinable, &Sensable, &Health)>,
) {
    for examine_event in examine_entity_events.messages.iter_mut() {
        let entity_reference = Entity::from_bits(examine_event.examine_entity_bits);

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

                match &health_component.health_container {
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

pub fn finalize_examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    mut net: EventWriter<NetConnExamine>,
) {
    for event in examine_map_events.messages.iter_mut() {
        event.message = event.message.to_string() + END_ASTRIX;

        net.send(NetConnExamine {
            handle: event.handle,
            message: ReliableServerMessage::ChatMessage(event.message.clone()),
        });
    }

    examine_map_events.messages.clear();
}

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

pub const END_ASTRIX: &str = concatcp!("\n", ASTRIX, "[/font]");
