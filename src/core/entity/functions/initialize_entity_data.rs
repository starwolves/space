use bevy_ecs::system::ResMut;

use crate::core::entity::resources::{EntityDataProperties, EntityDataResource};

pub fn initialize_entity_data(
    entity_data: &mut ResMut<EntityDataResource>,
    entity_properties: EntityDataProperties,
) {
    entity_data
        .id_to_name
        .insert(entity_properties.id, entity_properties.name.clone());
    entity_data
        .name_to_id
        .insert(entity_properties.name.clone(), entity_properties.id);
    entity_data.data.push(entity_properties);
}
