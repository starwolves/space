use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Commands, Transform}};
use bevy_rapier3d::prelude::{ActiveEvents, ColliderBundle, ColliderFlags, ColliderShape, InteractionGroups, RigidBodyBundle, RigidBodyType};

use crate::space_core::{components::{air_lock::{AccessLightsStatus, AirLock, AirLockStatus}, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::Examinable, pawn::SpaceAccessEnum, sensable::Sensable, static_transform::StaticTransform}, functions::{converters::transform_to_isometry::transform_to_isometry, entity::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, new_chat_message::{FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT}}}};

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

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

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

        let examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]*******\n"
        + "A security air lock. It will only grant access to those authorised to use it."
        + "[font=" + FURTHER_ITALIC_FONT + "]\n\nIt is in perfect shape and fully operational.[/font]"
        + "\n*******[/font]";

        commands.spawn_bundle(rigid_body_component).insert_bundle(collider_component).insert_bundle((
            static_transform_component,
            Sensable{
                is_audible : false,
                is_light:false,
                sensed_by: vec![],
                sensed_by_cached: vec![],
                always_sensed : false
            },
            AirLock {
                status : AirLockStatus::Closed,
                access_lights : AccessLightsStatus::Neutral,
                access_permissions : vec![SpaceAccessEnum::Security]
            },
            EntityData{
                entity_class: "entity".to_string(),
                entity_type: "securityAirLock1".to_string(),
                entity_group: EntityGroup::AirLock
            },
            EntityUpdates{
                updates: entity_updates_map,
                changed_parameters: vec![],
                excluded_handles:HashMap::new(),
                updates_difference: HashMap::new(),
            },
            Examinable {
                description: examine_text,
                name : "a security airlock".to_string()
            }
        ));

    }

}
