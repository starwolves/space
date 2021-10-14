
use std::collections::{BTreeMap, HashMap};

use bevy::{math::Vec3, prelude::{Commands, Transform}};
use bevy_rapier3d::prelude::{ActiveEvents, ColliderBundle, ColliderFlags, ColliderShape, InteractionGroups, RigidBodyBundle, RigidBodyType};

use crate::space_core::{components::{air_lock::{AirLock}, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::Examinable, health::{Health, HealthFlag}, pawn::SpaceAccessEnum, sensable::Sensable, static_transform::StaticTransform}, functions::{converters::transform_to_isometry::transform_to_isometry, entity::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, new_chat_message::{FURTHER_ITALIC_FONT, HEALTHY_COLOR}}}};

pub struct SecurityAirlockBundle;

impl SecurityAirlockBundle {

    pub fn spawn(
        entity_transform : Transform,
        commands : &mut Commands,
        _correct_transform : bool,
    ) {

        let static_transform_component = StaticTransform {
            transform: entity_transform
        };


        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: transform_to_isometry(entity_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider_component = ColliderBundle {
            shape: ColliderShape::cuboid(1.,1.,0.2),
            position: Vec3::new(0., 1., 0.).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                active_events: (ActiveEvents::CONTACT_EVENTS),
                ..Default::default()
            },
            ..Default::default()
        };

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, "A security air lock. It will only grant access to those authorised to use it.".to_string());
        examine_map.insert(1, "[font=".to_string() + FURTHER_ITALIC_FONT + "][color=" + HEALTHY_COLOR + "]It is fully operational.[/color][/font]");

        let mut health_flags = HashMap::new();

        health_flags.insert(0, HealthFlag::ArmourPlated);

        commands.spawn_bundle(rigid_body_component).insert_bundle(collider_component).insert_bundle((
            static_transform_component,
            Sensable::default(),
            AirLock {
                access_permissions : vec![SpaceAccessEnum::Security],
                ..Default::default()
            },
            EntityData{
                entity_class: "entity".to_string(),
                entity_type: "securityAirLock1".to_string(),
                entity_group: EntityGroup::AirLock
            },
            EntityUpdates::default(),
            Examinable {
                a_name : "a security airlock".to_string(),
                name : "security airlock".to_string(),
                assigned_texts: examine_map,
                ..Default::default()
            },
            Health {
                is_obstacle : true,
                ..Default::default()
            },
        ));

    }

}
