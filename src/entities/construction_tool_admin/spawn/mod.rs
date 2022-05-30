pub mod entity_bundle;
pub mod inventory_item_bundle;
pub mod rigidbody_bundle;

use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        entity::{
            resources::SpawnData,
            spawn::{base_entity_builder, showcase_builder, BaseEntityData, ShowCaseBuilderData},
        },
        inventory_item::spawn::{inventory_item_builder, InventoryBuilderData},
        rigid_body::spawn::{rigidbody_builder, RigidBodySpawnData},
    },
    entities::construction_tool_admin::components::ConstructionTool,
};

use entity_bundle::entity_bundle;
use inventory_item_bundle::inventory_item_bundle;
use rigidbody_bundle::rigidbody_bundle;

pub struct ConstructionToolBundle;

impl ConstructionToolBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let entity = spawn_data.commands.spawn().id();

        let default_transform = Transform::identity();

        let rigidbody_bundle = rigidbody_bundle();
        let inventory_item_bundle =
            inventory_item_bundle(spawn_data.held_data_option, default_transform);
        let entity_bundle = entity_bundle(default_transform);

        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
        }

        rigidbody_builder(
            &mut spawn_data.commands,
            entity,
            RigidBodySpawnData {
                rigidbody_dynamic: true,
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
                dynamicbody: true,
                entity_type: entity_bundle.entity_name.clone(),
                examinable: entity_bundle.examinable,
                is_item_in_storage: spawn_data.held_data_option.is_some(),

                is_showcase: spawn_data.showcase_data_option.is_some(),
                ..Default::default()
            },
        );

        inventory_item_builder(
            &mut spawn_data.commands,
            entity,
            InventoryBuilderData {
                inventory_item: inventory_item_bundle.inventory_item,
                holder_entity_option: spawn_data.held_data_option,
            },
        );

        showcase_builder(
            &mut spawn_data.commands,
            entity,
            spawn_data.showcase_data_option,
            ShowCaseBuilderData {
                entity_type: entity_bundle.entity_name,
                entity_updates: HashMap::new(),
            },
        );

        spawn_data
            .commands
            .entity(entity)
            .insert(ConstructionTool::default());

        entity
    }
}
