use std::collections::HashMap;

use bevy::prelude::{Changed, Query};
use entity::entity_data::{get_entity_update_difference, EntityUpdates};
use networking::server::EntityUpdateData;

use super::resources::{AccessLightsStatus, AirLock, AirLockStatus};

/// Air lock entity update for Godot clients.
#[cfg(feature = "server")]
pub(crate) fn air_lock_update(
    mut updated_air_locks: Query<(&AirLock, &mut EntityUpdates), Changed<AirLock>>,
) {
    for (air_lock_component, mut entity_updates_component) in updated_air_locks.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        let mut animation_tree_data = HashMap::new();

        animation_tree_data.insert("blend_speed".to_string(), EntityUpdateData::Float(0.6));

        match air_lock_component.status {
            AirLockStatus::Open => {
                animation_tree_data
                    .insert("blend_position".to_string(), EntityUpdateData::Float(1.));
            }
            AirLockStatus::Closed => {
                animation_tree_data
                    .insert("blend_position".to_string(), EntityUpdateData::Float(-1.));
            }
        }

        entity_updates_component.updates.insert(
            "animationTree1>>parameters/blend_position".to_string(),
            animation_tree_data,
        );

        let mut door_left_data = HashMap::new();
        let mut door_right_data = HashMap::new();

        match air_lock_component.access_lights {
            AccessLightsStatus::Neutral => {
                door_left_data.insert(
                    "emissiveTexture".to_string(),
                    EntityUpdateData::String(
                        "/content/entities/securityAirLock1/doorLeftEmissive.png".to_string(),
                    ),
                );
                door_right_data.insert(
                    "emissiveTexture".to_string(),
                    EntityUpdateData::String(
                        "/content/entities/securityAirLock1/doorRightEmissive.png".to_string(),
                    ),
                );
            }
            AccessLightsStatus::Granted => {
                door_left_data.insert(
                    "emissiveTexture".to_string(),
                    EntityUpdateData::String(
                        "/content/entities/securityAirLock1/allowedDoorLeftEmissive.png"
                            .to_string(),
                    ),
                );
                door_right_data.insert(
                    "emissiveTexture".to_string(),
                    EntityUpdateData::String(
                        "/content/entities/securityAirLock1/allowedDoorRightEmissive.png"
                            .to_string(),
                    ),
                );
            }
            AccessLightsStatus::Denied => {
                door_left_data.insert(
                    "emissiveTexture".to_string(),
                    EntityUpdateData::String(
                        "/content/entities/securityAirLock1/deniedDoorLeftEmissive.png".to_string(),
                    ),
                );
                door_right_data.insert(
                    "emissiveTexture".to_string(),
                    EntityUpdateData::String(
                        "/content/entities/securityAirLock1/deniedDoorRightEmissive.png"
                            .to_string(),
                    ),
                );
            }
        }

        entity_updates_component
            .updates
            .insert("doorLeft++material".to_string(), door_left_data);
        entity_updates_component
            .updates
            .insert("doorRight++material".to_string(), door_right_data);

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
