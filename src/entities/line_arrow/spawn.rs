use std::collections::BTreeMap;

use bevy_core::Timer;
use bevy_ecs::entity::Entity;
use bevy_log::warn;
use bevy_transform::components::Transform;

use crate::core::{
    entity::{
        components::{EntityData, EntityUpdates},
        resources::SpawnData,
    },
    examinable::components::{Examinable, RichName},
    networking::resources::ConsoleCommandVariantValues,
    physics::components::{WorldMode, WorldModes},
    rigid_body::components::{CachedBroadcastTransform, DefaultTransform},
    sensable::components::Sensable,
};

use super::components::{LineArrow, PointArrow};

pub struct LineArrowBundle;

impl LineArrowBundle {
    pub fn spawn(spawn_data: SpawnData) -> Entity {
        let mut this_transform;
        let default_transform = Transform::identity();

        this_transform = spawn_data.entity_transform;

        if spawn_data.correct_transform {
            this_transform.rotation = default_transform.rotation;
        }

        let duration;

        match spawn_data.properties.get("duration").unwrap() {
            ConsoleCommandVariantValues::Int(dur) => {
                duration = dur;
            }
            _ => {
                warn!("invalid duration type");
                return Entity::from_bits(0);
            }
        }

        let template_examine_text =
            "A holographic arrow without additional data points.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        let mut builder = spawn_data.commands.spawn_bundle((
            EntityData {
                entity_class: "entity".to_string(),
                entity_name: "lineArrow".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
            this_transform,
            CachedBroadcastTransform::default(),
            Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "arrow".to_string(),
                    n: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            DefaultTransform {
                transform: default_transform,
            },
            LineArrow,
            PointArrow {
                timer: Timer::from_seconds(*duration as f32, false),
            },
        ));

        let entity_id = builder.id();
        builder.insert(Sensable::default());

        entity_id
    }
}
