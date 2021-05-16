use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{air_lock::{AirLock,AirLockStatus}, entity_updates::EntityUpdates}, structs::network_messages::EntityUpdateData};

pub fn air_lock_update(
    mut updated_air_locks: Query<(&AirLock, &mut EntityUpdates), Changed<AirLock>>,
) {

    for (air_lock_component, mut entity_updates_component) in updated_air_locks.iter_mut() {

        let mut animation_tree_data = HashMap::new();

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

    }

}
