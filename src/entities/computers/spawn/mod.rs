use bevy_ecs::entity::Entity;
use bevy_log::warn;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        resources::SpawnData,
        spawn::{base_entity_builder, BaseEntityData},
    },
    networking::resources::ConsoleCommandVariantValues,
    rigid_body::spawn::{rigidbody_builder, RigidBodySpawnData},
};

pub mod entity_bundle;
pub mod rigidbody_bundle;

use entity_bundle::entity_bundle;
use rigidbody_bundle::rigidbody_bundle;

use super::components::Computer;

pub const DEFAULT_AIR_LOCK_Y: f32 = 1.;
pub struct ComputerBundle;

impl ComputerBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let computer_type;

        match spawn_data.properties.get("computerType") {
            Some(x) => match x {
                ConsoleCommandVariantValues::String(s) => {
                    computer_type = s.to_string();
                }
                _ => {
                    warn!("computerType had incorrect variable type!");
                    computer_type = "".to_string();
                }
            },
            None => {
                warn!("computerType not found.");
                computer_type = "".to_string();
            }
        }

        let default_transform = Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(1., 1., 1.),
            Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
            Vec3::new(0., 0.355, 0.),
        ));

        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
        }

        let entity = spawn_data.commands.spawn().id();

        let rigidbody_bundle = rigidbody_bundle();
        let entity_bundle = entity_bundle(default_transform);

        rigidbody_builder(
            &mut spawn_data.commands,
            entity,
            RigidBodySpawnData {
                rigidbody_dynamic: false,
                rigid_transform: spawn_data.entity_transform,
                entity_is_stored_item: spawn_data.held_data_option.is_some(),
                collider: rigidbody_bundle.collider,
                collider_transform: rigidbody_bundle.collider_transform,
                collider_friction: rigidbody_bundle.collider_friction,
                ..Default::default()
            },
        );

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

        spawn_data
            .commands
            .entity(entity)
            .insert(Computer { computer_type });

        entity
    }
}
