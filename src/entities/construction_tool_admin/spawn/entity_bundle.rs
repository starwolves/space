use std::collections::BTreeMap;

use bevy_math::Quat;

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle() -> EntityBundle {
    let mut examine_map = BTreeMap::new();
    examine_map.insert(
        0,
        "A construction tool. Use this to construct or deconstruct ship hull cells.".to_string(),
    );
    EntityBundle {
        default_rotation: Quat::IDENTITY,
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "admin construction tool".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "constructionTool".to_string(),
    }
}
