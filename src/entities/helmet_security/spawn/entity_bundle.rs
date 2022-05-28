use bevy_transform::prelude::Transform;
use std::collections::BTreeMap;

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle(default_transform: Transform) -> EntityBundle {
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
                name: "security helmet".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "helmetSecurity".to_string(),
    }
}
