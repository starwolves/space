use std::collections::BTreeMap;

use bevy::prelude::{Added, Component, Query};
use entity::{
    entity_data::EntityData,
    examine::{Examinable, RichName},
};
use text_api::core::{FURTHER_ITALIC_FONT, HEALTHY_COLOR};

use super::spawn::BRIDGE_COMPUTER_ENTITY_NAME;

/// On a computer spawn.
#[cfg(feature = "server")]
pub(crate) fn computer_added(
    mut computers: Query<(&EntityData, &mut Examinable), Added<Computer>>,
) {
    for (entity_data_component, mut examinable_component) in computers.iter_mut() {
        if entity_data_component.entity_type.to_string() == BRIDGE_COMPUTER_ENTITY_NAME {
            examinable_component.name = RichName {
                name: "bridge computer".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "A computer used by bridge personnel.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        }
    }
}

/// The computer component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Computer;
