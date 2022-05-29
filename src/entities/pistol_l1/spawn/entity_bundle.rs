use std::collections::BTreeMap;

use bevy_transform::prelude::Transform;

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle(default_transform: Transform) -> EntityBundle {
    let mut examine_map = BTreeMap::new();
    examine_map.insert(
        0,
        "A standard issue laser pistol. It is a lethal weapon.".to_string(),
    );

    EntityBundle {
        default_transform,
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "laser pistol".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "pistolL1".to_string(),

        ..Default::default()
    }
}
