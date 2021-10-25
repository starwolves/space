use bevy::prelude::{EventReader, EventWriter};

use crate::space_core::{events::general::{examine_entity::InputExamineEntity, examine_map::InputExamineMap, input_tab_action::InputTabAction}, resources::doryen_fov::Vec3Int};

pub fn tab_action(

    mut events : EventReader<InputTabAction>,
    mut event_examine_entity : EventWriter<InputExamineEntity>,
    mut event_examine_map : EventWriter<InputExamineMap>,

) {

    for event in events.iter() {

        if event.tab_id == "examine" {

            match event.target_entity_option {
                Some(entity_bits) => {

                    event_examine_entity.send(InputExamineEntity{
                        handle: event.handle,
                        examine_entity_bits: entity_bits,
                    });

                },
                None => {

                    match &event.target_cell_option {
                        Some((gridmap_type, idx, idy, idz)) => {

                            event_examine_map.send(InputExamineMap{
                                handle: event.handle,
                                entity: event.player_entity,
                                gridmap_type: gridmap_type.clone(),
                                gridmap_cell_id: Vec3Int {
                                    x: *idx,
                                    y: *idy,
                                    z: *idz,
                                },
                            });

                        },
                        None => {},
                    }

                },
            }

        }

    }

}
