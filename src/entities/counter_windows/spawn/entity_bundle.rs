use std::collections::BTreeMap;

use bevy_transform::prelude::Transform;

use crate::core::{
    chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle(
    default_transform: Transform,
    department_name: &str,
    entity_name: String,
) -> EntityBundle {
    let mut examine_map = BTreeMap::new();
    examine_map.insert(
        0,
        "An airtight ".to_string()
            + department_name
            + " window. It will only grant access to those authorised to use it.",
    );
    examine_map.insert(
        1,
        "[font=".to_string()
            + FURTHER_ITALIC_FONT
            + "][color="
            + HEALTHY_COLOR
            + "]It is fully operational.[/color][/font]",
    );
    EntityBundle {
        default_transform,
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: department_name.to_string() + " window",
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name,
        ..Default::default()
    }
}
