use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{entity_updates::EntityUpdates, world_mode::{WorldMode,WorldModes}}, functions::get_entity_update_difference::get_entity_update_difference, structs::network_messages::EntityUpdateData};

pub fn world_mode_update(
    mut updated_entities: Query<(&WorldMode, &mut EntityUpdates), Changed<WorldMode>>,
) {
    
    for (world_mode_component, mut entity_updates_component) in updated_entities.iter_mut() {

        let old_entity_updates = entity_updates_component.updates.clone();

        let world_mode;

        match world_mode_component.mode {
            WorldModes::Static => {
                world_mode = "static";
            }
            WorldModes::Kinematic => {
                world_mode = "kinematic";
            }
            WorldModes::Physics => {
                world_mode = "physics";
            }
            WorldModes::Worn => {
                world_mode = "worn";
            }
            WorldModes::Held => {
                world_mode = "held";
            },
        };

        let entity_updates = entity_updates_component.updates
        .get_mut(&".".to_string()).unwrap();

        entity_updates.insert("world_mode".to_string(), EntityUpdateData::String(world_mode.to_string()));

        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference = difference_updates;

    }

}
