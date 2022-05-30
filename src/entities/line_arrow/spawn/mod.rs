pub mod entity_bundle;

use bevy_core::Timer;
use bevy_ecs::entity::Entity;
use bevy_log::warn;
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        resources::SpawnData,
        spawn::{base_entity_builder, BaseEntityData},
    },
    networking::resources::ConsoleCommandVariantValues,
    physics::components::{WorldMode, WorldModes},
};

use entity_bundle::entity_bundle;

use super::components::{LineArrow, PointArrow};

pub struct LineArrowBundle;

impl LineArrowBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let default_transform = Transform::identity();

        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
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

        let entity = spawn_data.commands.spawn().id();

        let default_transform = Transform::identity();

        let entity_bundle = entity_bundle(default_transform);

        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
        }

        base_entity_builder(
            &mut spawn_data.commands,
            entity,
            BaseEntityData {
                dynamicbody: false,
                entity_type: entity_bundle.entity_name.clone(),
                examinable: entity_bundle.examinable,
                ..Default::default()
            },
        );

        spawn_data.commands.entity(entity).insert_bundle((
            spawn_data.entity_transform,
            LineArrow,
            PointArrow {
                timer: Timer::from_seconds(*duration as f32, false),
            },
            WorldMode {
                mode: WorldModes::Static,
            },
        ));

        entity
    }
}
