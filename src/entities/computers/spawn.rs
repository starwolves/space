use std::collections::{BTreeMap, HashMap};

use bevy_ecs::entity::Entity;
use bevy_hierarchy::BuildChildren;
use bevy_log::warn;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody,
};
use bevy_transform::components::Transform;

use crate::{
    core::{
        entity::{
            components::{EntityData, EntityUpdates},
            resources::SpawnData,
        },
        examinable::components::{Examinable, RichName},
        health::components::{DamageFlag, Health},
        networking::resources::ConsoleCommandVariantValues,
        physics::{
            components::{WorldMode, WorldModes},
            functions::{get_bit_masks, ColliderGroup},
        },
        rigid_body::components::RigidBodyData,
        sensable::components::Sensable,
    },
    entities::computers::components::Computer,
};

pub const STANDARD_BODY_FRICTION: f32 = 0.125;

pub struct ComputerBundle;

impl ComputerBundle {
    pub fn spawn(spawn_data: SpawnData) -> Entity {
        let computer_type;

        match spawn_data.properties.get("computerType").unwrap() {
            ConsoleCommandVariantValues::String(s) => {
                computer_type = s.to_string();
            }
            _ => {
                warn!("computerType had incorrect variable type!");
                computer_type = "".to_string();
            }
        }

        let mut this_transform;
        let default_transform = Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(1., 1., 1.),
            Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
            Vec3::new(0., 0.355, 0.),
        ));

        this_transform = spawn_data.entity_transform;

        if spawn_data.correct_transform {
            this_transform.rotation = default_transform.rotation;
        }

        let friction_val = STANDARD_BODY_FRICTION;
        let friction_combine_rule = CoefficientCombineRule::Min;

        let mut t = Transform::from_translation(this_transform.translation);
        t.rotation = this_transform.rotation;
        let mut friction = Friction::coefficient(friction_val);
        friction.combine_rule = friction_combine_rule;

        let shape = Collider::cuboid(1., 0.7, 1.);

        let collider_position = Vec3::new(0., 0., 0.).into();
        let rigid_body;

        rigid_body = RigidBody::Fixed;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut builder = spawn_data.commands.spawn();

        builder.insert(rigid_body).insert(t);

        builder.with_children(|children| {
            children
                .spawn()
                .insert(shape)
                .insert(Transform::from_translation(collider_position))
                .insert(friction)
                .insert(CollisionGroups::new(masks.0, masks.1));
        });

        let template_examine_text = "A computer used by bridge personnel.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        let entity_id = builder.id();

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        builder.insert_bundle((
            EntityData {
                entity_class: "entity".to_string(),
                entity_name: "bridgeComputer".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
            Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "bridge computer".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            Computer { computer_type },
            this_transform,
            RigidBodyData {
                friction: friction.coefficient,
                friction_combine_rule: friction.combine_rule,
            },
        ));

        builder.insert_bundle((Sensable::default(), Health::default()));

        entity_id
    }
}
