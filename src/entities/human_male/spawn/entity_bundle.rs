use bevy_transform::prelude::Transform;
use std::collections::BTreeMap;

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle(default_transform: Transform, character_name: String) -> EntityBundle {
    let mut examine_map = BTreeMap::new();
    examine_map.insert(
        0,
        "A standard issue helmet used by Security Officers.".to_string(),
    );
    EntityBundle {
        default_transform,
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: character_name.clone(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "humanMale".to_string(),
        ..Default::default()
    }
}
