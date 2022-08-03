use std::collections::BTreeMap;

use api::{
    chat::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
    entity_updates::EntityData,
    examinable::{Examinable, RichName},
};
use bevy::prelude::{Added, Component, Query};

use super::spawn::BRIDGE_COMPUTER_ENTITY_NAME;

pub fn computer_added(mut computers: Query<(&EntityData, &mut Examinable), Added<Computer>>) {
    for (entity_data_component, mut examinable_component) in computers.iter_mut() {
        if entity_data_component.entity_name == BRIDGE_COMPUTER_ENTITY_NAME {
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

#[derive(Component)]
pub struct Computer {
    pub computer_type: String,
}
