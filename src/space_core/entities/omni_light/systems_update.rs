use bevy::prelude::{Changed, Query};


use crate::space_core::{entities::omni_light::components::OmniLight, generics::{entity::{components::EntityUpdates, functions::get_entity_update_difference::get_entity_update_difference}, networking::resources::EntityUpdateData}};

pub fn omni_light_update(
    mut updated_omni_lights: Query<(&OmniLight, &mut EntityUpdates), Changed<OmniLight>>,
) {

    for (omni_light_component, mut entity_updates_component) in updated_omni_lights.iter_mut() {

        let old_entity_updates = entity_updates_component.updates.clone();

        let entity_updates = entity_updates_component.updates
        .get_mut(&".".to_string()).unwrap();

        entity_updates.insert(
            "omni_attenuation".to_string(),
            EntityUpdateData::Float(omni_light_component.omni_attenuation)
        );
        entity_updates.insert(
            "omni_range".to_string(), 
            EntityUpdateData::Float(omni_light_component.omni_range)
        );
        entity_updates.insert(
            "omni_shadow_detail".to_string(), 
            EntityUpdateData::UInt8(omni_light_component.omni_shadow_detail)
        );
        entity_updates.insert(
            "omni_shadow_mode".to_string(), 
            EntityUpdateData::UInt8(omni_light_component.omni_shadow_mode)
        );
        entity_updates.insert(
            "bake_mode".to_string(), 
            EntityUpdateData::UInt8(omni_light_component.bake_mode)
        );
        entity_updates.insert(
            "color".to_string(), 
            EntityUpdateData::Color(omni_light_component.color)
        );
        entity_updates.insert(
            "cull_mask".to_string(), 
            EntityUpdateData::Int(omni_light_component.cull_mask)
        );
        entity_updates.insert(
            "light_energy".to_string(), 
            EntityUpdateData::Float(omni_light_component.light_energy)
        );
        entity_updates.insert(
            "light_indirect_energy".to_string(), 
            EntityUpdateData::Float(omni_light_component.light_indirect_energy)
        );
        entity_updates.insert(
            "negative".to_string(), 
            EntityUpdateData::Bool(omni_light_component.negative)
        );
        entity_updates.insert(
            "light_specular".to_string(), 
            EntityUpdateData::Float(omni_light_component.light_specular)
        );
        entity_updates.insert(
            "shadow_bias".to_string(), 
            EntityUpdateData::Float(omni_light_component.shadow_bias)
        );
        entity_updates.insert(
            "shadow_color".to_string(), 
            EntityUpdateData::Color(omni_light_component.shadow_color)
        );
        entity_updates.insert(
            "shadow_contact".to_string(), 
            EntityUpdateData::Float(omni_light_component.shadow_contact)
        );
        entity_updates.insert(
            "shadow".to_string(),
            EntityUpdateData::Bool(omni_light_component.shadow)
        );
        entity_updates.insert(
            "shadow_reverse_cull_face".to_string(), 
            EntityUpdateData::Bool(omni_light_component.shadow_reverse_cull_face)
        );

        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference.push(difference_updates);

    }

}
