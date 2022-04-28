use std::collections::BTreeMap;

use bevy_ecs::{prelude::Added, system::Query};

use crate::core::{
    chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
    entity::components::EntityData,
    examinable::components::{Examinable, RichName},
};

use super::components::Computer;

pub fn computer_added(mut computers: Query<(&EntityData, &mut Examinable), Added<Computer>>) {
    for (entity_data_component, mut examinable_component) in computers.iter_mut() {
        if entity_data_component.entity_name == "bridgeComputer" {
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
