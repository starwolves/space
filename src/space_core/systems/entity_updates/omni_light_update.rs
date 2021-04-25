use bevy::prelude::{Changed, Query};
use std::collections::HashMap;


use crate::space_core::{components::{
        omni_light::OmniLight,
        entity_updates::EntityUpdates
    }, structs::network_messages::EntityUpdateData};

pub fn omni_light_update(
    mut updated_omni_lights: Query<(&OmniLight, &mut EntityUpdates), Changed<OmniLight>>,
) {

    for (omni_light_component, mut entity_updates_component) in updated_omni_lights.iter_mut() {

        let mut omni_light_data = HashMap::new();

        omni_light_data.insert(
            "omni_attenuation".to_string(),
            EntityUpdateData::Float(omni_light_component.omni_attenuation)
        );
        omni_light_data.insert(
            "omni_range".to_string(), 
            EntityUpdateData::Float(omni_light_component.omni_range)
        );
        omni_light_data.insert(
            "omni_shadow_detail".to_string(), 
            EntityUpdateData::UInt8(omni_light_component.omni_shadow_detail)
        );
        omni_light_data.insert(
            "omni_shadow_mode".to_string(), 
            EntityUpdateData::UInt8(omni_light_component.omni_shadow_mode)
        );
        omni_light_data.insert(
            "bake_mode".to_string(), 
            EntityUpdateData::UInt8(omni_light_component.bake_mode)
        );
        omni_light_data.insert(
            "color".to_string(), 
            EntityUpdateData::Color(omni_light_component.color)
        );
        omni_light_data.insert(
            "cull_mask".to_string(), 
            EntityUpdateData::Int(omni_light_component.cull_mask)
        );
        omni_light_data.insert(
            "light_energy".to_string(), 
            EntityUpdateData::Float(omni_light_component.light_energy)
        );
        omni_light_data.insert(
            "light_indirect_energy".to_string(), 
            EntityUpdateData::Float(omni_light_component.light_indirect_energy)
        );
        omni_light_data.insert(
            "negative".to_string(), 
            EntityUpdateData::Bool(omni_light_component.negative)
        );
        omni_light_data.insert(
            "light_specular".to_string(), 
            EntityUpdateData::Float(omni_light_component.light_specular)
        );
        omni_light_data.insert(
            "shadow_bias".to_string(), 
            EntityUpdateData::Float(omni_light_component.shadow_bias)
        );
        omni_light_data.insert(
            "shadow_color".to_string(), 
            EntityUpdateData::Color(omni_light_component.shadow_color)
        );
        omni_light_data.insert(
            "shadow_contact".to_string(), 
            EntityUpdateData::Float(omni_light_component.shadow_contact)
        );
        omni_light_data.insert(
            "shadow".to_string(), 
            EntityUpdateData::Bool(omni_light_component.shadow)
        );
        omni_light_data.insert(
            "shadow_reverse_cull_face".to_string(), 
            EntityUpdateData::Bool(omni_light_component.shadow_reverse_cull_face)
        );

        entity_updates_component.updates.insert(".".to_string(), omni_light_data);


    }

}
