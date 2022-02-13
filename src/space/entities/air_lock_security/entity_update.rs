use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space::{entities::air_lock_security::components::{AirLock, AirLockStatus, AccessLightsStatus}, core::{entity::{components::EntityUpdates, functions::get_entity_update_difference::get_entity_update_difference}, networking::resources::EntityUpdateData},};

pub fn air_lock_update(
    mut updated_air_locks: Query<(&AirLock, &mut EntityUpdates), Changed<AirLock>>,
) {

    for (air_lock_component, mut entity_updates_component) in updated_air_locks.iter_mut() {

        let old_entity_updates = entity_updates_component.updates.clone();

        let mut animation_tree_data = HashMap::new();

        animation_tree_data.insert(
        "blend_speed".to_string(),
        EntityUpdateData::Float(0.6)
        );

        match air_lock_component.status {
            AirLockStatus::Open => {
                animation_tree_data.insert(
                "blend_position".to_string(),
                EntityUpdateData::Float(1.)
                );
            }
            AirLockStatus::Closed => {
                animation_tree_data.insert(
                "blend_position".to_string(),
                EntityUpdateData::Float(-1.)
                );
            }
        }

        entity_updates_component.updates.insert(
            "animationTree1>>parameters/blend_position".to_string(),
            animation_tree_data
        );

        let mut door_left_data = HashMap::new();
        let mut door_right_data = HashMap::new();


        match air_lock_component.access_lights {
            AccessLightsStatus::Neutral => {
                door_left_data.insert(
                "emissiveTexture".to_string(),
                EntityUpdateData::String("doorLeftEmissive".to_string())
                );
                door_right_data.insert(
                "emissiveTexture".to_string(),
                EntityUpdateData::String("doorRightEmissive".to_string())
                );
            }
            AccessLightsStatus::Granted => {
                door_left_data.insert(
                "emissiveTexture".to_string(),
                EntityUpdateData::String("allowedDoorLeftEmissive".to_string())
                );
                door_right_data.insert(
                "emissiveTexture".to_string(),
                EntityUpdateData::String("allowedDoorRightEmissive".to_string())
                );
            }
            AccessLightsStatus::Denied => {
                door_left_data.insert(
                "emissiveTexture".to_string(),
                EntityUpdateData::String("deniedDoorLeftEmissive".to_string())
                );
                door_right_data.insert(
                "emissiveTexture".to_string(),
                EntityUpdateData::String("deniedDoorRightEmissive".to_string())
                );
            }
        }


        entity_updates_component.updates.insert(
            "doorLeft++material".to_string(),
            door_left_data
        );
        entity_updates_component.updates.insert(
            "doorRight++material".to_string(),
            door_right_data
        );

        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference.push(difference_updates);


    }

}
