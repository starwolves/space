use actions::core::BuildingActions;
use bevy::prelude::{warn, Query, ResMut, Transform};
use gridmap::grid::cell_id_to_world;
use inventory_api::core::Inventory;
use math::grid::Vec3Int;
use pawn::pawn::REACH_DISTANCE;

/// Inventory item action prerequisite check for pickup action.
pub(crate) fn pickup_prerequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    transforms: Query<&Transform>,
    inventories: Query<&Inventory>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "actions::inventory/pickup" {
                let examiner_inventory_option = inventories.get(building.action_taker);

                let examiner_transform;

                match transforms.get(building.action_taker) {
                    Ok(t) => {
                        examiner_transform = t;
                    }
                    Err(_rr) => {
                        warn!("Couldnt find transform of examining entity!");
                        continue;
                    }
                }

                let distance;
                let start_pos;
                let end_pos = examiner_transform.translation;

                match building.target_entity_option.clone() {
                    Some(target_entity_bits) => match transforms.get(target_entity_bits) {
                        Ok(rigid_body_position) => {
                            start_pos = rigid_body_position.translation;
                        }
                        Err(_) => {
                            continue;
                        }
                    },
                    None => {
                        let cell_data;
                        match building.target_cell_option.as_ref() {
                            Some(v) => {
                                cell_data = v;
                            }
                            None => {
                                continue;
                            }
                        }
                        start_pos = cell_id_to_world(Vec3Int {
                            x: cell_data.0.x,
                            y: cell_data.0.y,
                            z: cell_data.0.z,
                        });
                    }
                }

                let mut inventory_ok = true;

                match examiner_inventory_option {
                    Ok(i) => {
                        inventory_ok = i.get_active_slot_entity().is_none();
                    }
                    Err(_) => {}
                }

                distance = start_pos.distance(end_pos);

                if distance < REACH_DISTANCE && inventory_ok {
                    action.approve();
                }
            }
        }
    }
}
