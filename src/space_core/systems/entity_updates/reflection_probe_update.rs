use bevy::prelude::{Changed, Query};


use crate::space_core::{components::{entity_updates::EntityUpdates, reflection_probe::ReflectionProbe}, structs::network_messages::EntityUpdateData};

pub fn reflection_probe_update(
    mut updated_reflection_probes: Query<(&ReflectionProbe, &mut EntityUpdates), Changed<ReflectionProbe>>,
) {
    
    for (reflection_probe_component, mut entity_updates_component) in updated_reflection_probes.iter_mut() {

        let entity_updates = entity_updates_component.updates
        .get_mut(&".".to_string()).unwrap();

    }


}