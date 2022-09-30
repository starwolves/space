use api::entity_updates::{get_entity_update_difference, EntityUpdateData, EntityUpdates};
use bevy::prelude::{Changed, Query};

use crate::core::ReflectionProbe;

/// Reflection probe entity update.
pub(crate) fn reflection_probe_update(
    mut updated_reflection_probes: Query<
        (&ReflectionProbe, &mut EntityUpdates),
        Changed<ReflectionProbe>,
    >,
) {
    for (reflection_probe_component, mut entity_updates_component) in
        updated_reflection_probes.iter_mut()
    {
        let old_entity_updates = entity_updates_component.updates.clone();

        let entity_updates = entity_updates_component
            .updates
            .get_mut(&".".to_string())
            .unwrap();

        entity_updates.insert(
            "projection_enabled".to_string(),
            EntityUpdateData::Bool(reflection_probe_component.projection_enabled),
        );
        entity_updates.insert(
            "cull_mask".to_string(),
            EntityUpdateData::Int(reflection_probe_component.cull_mask),
        );
        entity_updates.insert(
            "shadows_enabled".to_string(),
            EntityUpdateData::Bool(reflection_probe_component.shadows_enabled),
        );
        entity_updates.insert(
            "extents".to_string(),
            EntityUpdateData::Vec3(reflection_probe_component.extents),
        );
        entity_updates.insert(
            "intensity".to_string(),
            EntityUpdateData::Float(reflection_probe_component.intensity),
        );
        entity_updates.insert(
            "interior_ambient".to_string(),
            EntityUpdateData::Color(
                reflection_probe_component.interior_ambient.0,
                reflection_probe_component.interior_ambient.1,
                reflection_probe_component.interior_ambient.2,
                reflection_probe_component.interior_ambient.3,
            ),
        );
        entity_updates.insert(
            "interior_ambient_probe_contribution".to_string(),
            EntityUpdateData::Float(reflection_probe_component.interior_ambient_probe_contribution),
        );
        entity_updates.insert(
            "interior_ambient_energy".to_string(),
            EntityUpdateData::Float(reflection_probe_component.interior_ambient_energy),
        );
        entity_updates.insert(
            "set_as_interior".to_string(),
            EntityUpdateData::Bool(reflection_probe_component.set_as_interior),
        );
        entity_updates.insert(
            "max_distance".to_string(),
            EntityUpdateData::Float(reflection_probe_component.max_distance),
        );
        entity_updates.insert(
            "origin_offset".to_string(),
            EntityUpdateData::Vec3(reflection_probe_component.origin_offset),
        );
        entity_updates.insert(
            "update_mode".to_string(),
            EntityUpdateData::Int(reflection_probe_component.update_mode as i64),
        );

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
