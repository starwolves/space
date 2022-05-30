use std::collections::BTreeMap;

use bevy_transform::prelude::Transform;

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle(default_transform: Transform) -> EntityBundle {
    let template_examine_text = "A holographic arrow without additional data points.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);

    EntityBundle {
        default_transform,
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "arrow".to_string(),
                n: true,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "lineArrow".to_string(),
        ..Default::default()
    }
}
