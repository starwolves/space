use std::collections::BTreeMap;

use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        resources::SpawnData,
        spawn::{BaseEntityBundle, BaseEntitySummonable, NoData},
    },
    examinable::components::{Examinable, RichName},
};

use super::{ConstructionToolSummoner, CONSTRUCTION_TOOL_ENTITY_NAME};

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoData> for ConstructionToolSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A construction tool. Use this to construct or deconstruct ship hull cells."
                .to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "admin construction tool".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}
