use std::collections::BTreeMap;

use bevy_transform::prelude::Transform;

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
    health::components::Health,
};

pub fn entity_bundle(default_transform: Transform) -> EntityBundle {
    let template_examine_text = "A computer used by bridge personnel.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);

    EntityBundle {
        default_transform,
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "bridge computer".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "bridgeComputer".to_string(),
        health: Health {
            is_combat_obstacle: true,
            is_reach_obstacle: true,
            ..Default::default()
        },
        ..Default::default()
    }
}
