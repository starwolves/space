use std::collections::{BTreeMap, HashMap};

use bevy_app::EventWriter;
use bevy_ecs::{entity::Entity, system::Commands};
use bevy_log::warn;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderPosition,
    ColliderShape, InteractionGroups, RigidBodyActivation, RigidBodyBundle, RigidBodyForces,
    RigidBodyType,
};
use bevy_transform::components::Transform;

use crate::space::{
    core::{
        entity::{
            components::{EntityData, EntityUpdates, Showcase},
            events::NetShowcase,
            functions::transform_to_isometry::transform_to_isometry,
            resources::{SpawnHeldData, SpawnPawnData},
        },
        examinable::components::{Examinable, RichName},
        health::components::{DamageFlag, Health},
        networking::resources::{ConsoleCommandVariantValues, ReliableServerMessage},
        physics::{
            components::{WorldMode, WorldModes},
            functions::{get_bit_masks, ColliderGroup},
        },
        rigid_body::components::RigidBodyData,
        sensable::components::Sensable,
        static_body::components::StaticTransform,
    },
    entities::computers::components::Computer,
};

pub const STANDARD_BODY_FRICTION: f32 = 0.125;

pub struct ComputerBundle;

impl ComputerBundle {
    pub fn spawn(
        passed_transform: Transform,
        commands: &mut Commands,
        correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        _default_map_spawn: bool,
        properties: HashMap<String, ConsoleCommandVariantValues>,
    ) -> Entity {
        spawn_entity(
            commands,
            Some(passed_transform),
            false,
            None,
            false,
            None,
            &mut None,
            correct_transform,
            properties,
        )
    }
}

fn spawn_entity(
    commands: &mut Commands,

    passed_transform_option: Option<Transform>,

    held: bool,
    _holder_entity_option: Option<Entity>,

    showcase_instance: bool,
    showcase_handle_option: Option<u32>,

    net_showcase: &mut Option<&mut EventWriter<NetShowcase>>,

    correct_transform: bool,
    properties: HashMap<String, ConsoleCommandVariantValues>,
) -> Entity {
    let computer_type;

    match properties.get("computerType").unwrap() {
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

    match passed_transform_option {
        Some(transform) => {
            this_transform = transform;
        }
        None => {
            this_transform = default_transform;
        }
    }

    if correct_transform {
        this_transform.rotation = default_transform.rotation;
    }

    let rigid_body_component;
    let collider_component;

    let shape = ColliderShape::cuboid(1., 0.7, 1.);

    let collider_position: ColliderPosition = Vec3::new(0., 0., 0.).into();
    let friction = STANDARD_BODY_FRICTION;
    let friction_combine_rule = CoefficientCombineRule::Min;

    if held == false {
        rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: transform_to_isometry(this_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        collider_component = ColliderBundle {
            shape: shape.into(),
            position: collider_position.into(),
            material: ColliderMaterial {
                friction,
                friction_combine_rule,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };
    } else {
        rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: transform_to_isometry(this_transform).into(),
            forces: RigidBodyForces {
                gravity_scale: 0.,
                ..Default::default()
            }
            .into(),
            activation: RigidBodyActivation {
                sleeping: true,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::NoCollision);

        collider_component = ColliderBundle {
            shape: shape.into(),
            position: collider_position.into(),
            material: ColliderMaterial {
                friction,
                friction_combine_rule,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };
    }

    let template_examine_text = "A computer used by bridge personnel.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);

    let mut builder = commands.spawn_bundle(rigid_body_component);

    let entity_id = builder.id();

    let mut melee_damage_flags = HashMap::new();
    melee_damage_flags.insert(0, DamageFlag::SoftDamage);

    builder.insert_bundle(collider_component).insert_bundle((
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
        StaticTransform {
            transform: this_transform,
        },
        RigidBodyData {
            friction,
            friction_combine_rule,
        },
    ));

    if showcase_instance {
        let handle = showcase_handle_option.unwrap();
        builder.insert(Showcase { handle: handle });
        let entity_updates = HashMap::new();
        net_showcase.as_deref_mut().unwrap().send(NetShowcase {
            handle: handle,
            message: ReliableServerMessage::LoadEntity(
                "entity".to_string(),
                "bridgeComputer".to_string(),
                entity_updates,
                entity_id.to_bits(),
                true,
                "main".to_string(),
                "".to_string(),
                false,
            ),
        });
    } else {
        builder.insert_bundle((Sensable::default(), Health::default()));
    }

    entity_id
}
