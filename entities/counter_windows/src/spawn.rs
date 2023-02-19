use std::collections::BTreeMap;

use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{warn, Commands, EventReader, GlobalTransform, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction, Group};
use entity::{
    entity_data::{EntityData, EntityGroup},
    entity_macros::Identity,
    entity_types::EntityType,
    examine::{Examinable, RichName},
    health::Health,
    spawn::{BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData, SpawnEntity},
};
use pawn::pawn::ShipAuthorizationEnum;
use physics::physics::{get_bit_masks, ColliderGroup};
use physics::spawn::{RigidBodyBuilder, RigidBodyBundle};
use resources::content::SF_CONTENT_PREFIX;
use text_api::core::{FURTHER_ITALIC_FONT, HEALTHY_COLOR};

use super::counter_window_events::{CounterWindow, CounterWindowSensor};

pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

impl BaseEntityBuilder<NoData> for CounterWindowType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let entity_name = self.sub_type.clone();
        let department_name;

        if entity_name == SECURITY_COUNTER_WINDOW_ENTITY_NAME {
            department_name = "security";
        } else if entity_name == BRIDGE_COUNTER_WINDOW_ENTITY_NAME {
            department_name = "bridge";
        } else {
            warn!("Unrecognized counterwindow sub-type {}", entity_name);
            department_name = "ERR";
        }
        let mut examine_map = BTreeMap::new();

        examine_map.insert(
            0,
            "An airtight ".to_string()
                + department_name
                + " window. It will only grant access to those authorised to use it.",
        );
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );
        BaseEntityBundle {
            entity_type: Box::new(CounterWindowType::new()),
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: department_name.to_string() + " window",
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health {
                is_combat_obstacle: true,
                is_laser_obstacle: false,
                is_reach_obstacle: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl RigidBodyBuilder<NoData> for CounterWindowType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        RigidBodyBundle {
            collider: Collider::cuboid(0.1, 0.5, 1.),
            collider_transform: Transform::from_translation(Vec3::new(
                0.,
                COUNTER_WINDOW_COLLISION_Y,
                0.,
            )),
            collider_friction: friction,
            rigidbody_dynamic: false,
            ..Default::default()
        }
    }
}

use bevy_rapier3d::prelude::{ActiveEvents, CollisionGroups, RigidBody, Sensor};

pub const COUNTER_WINDOW_COLLISION_Y: f32 = 0.5;

#[derive(Clone, Identity)]
pub struct CounterWindowType {
    pub identifier: String,
    pub sub_type: String,
}

impl Default for CounterWindowType {
    fn default() -> Self {
        CounterWindowType {
            identifier: SF_CONTENT_PREFIX.to_string() + "counter_window",
            sub_type: SECURITY_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        }
    }
}

pub fn build_counter_windows<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    use entity::entity_data::BlankEntityType;

    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(CounterWindow {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            });

        let rigid_body = RigidBody::Fixed;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        let sensor = Sensor;

        commands
            .entity(spawn_event.spawn_data.entity)
            .with_children(|children| {
                children
                    .spawn(())
                    .insert(rigid_body)
                    .insert(GlobalTransform::IDENTITY)
                    .insert(Transform::IDENTITY)
                    .insert((
                        CounterWindowSensor {
                            parent: spawn_event.spawn_data.entity,
                        },
                        EntityData {
                            entity_type: Box::new(BlankEntityType::default()),
                            entity_group: EntityGroup::CounterWindowSensor,
                        },
                    ))
                    .with_children(|children| {
                        children
                            .spawn(())
                            .insert(Collider::cuboid(1., 1., 1.))
                            .insert(Transform::from_translation(Vec3::new(0., -1., 0.)))
                            .insert(GlobalTransform::default())
                            .insert(friction)
                            .insert(CollisionGroups::new(
                                Group::from_bits(masks.0).unwrap(),
                                Group::from_bits(masks.1).unwrap(),
                            ))
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(sensor);
                    });
            });
    }
}
use const_format::concatcp;
pub const SECURITY_COUNTER_WINDOW_ENTITY_NAME: &str =
    concatcp!(SF_CONTENT_PREFIX, "securityCounterWindow");
pub const BRIDGE_COUNTER_WINDOW_ENTITY_NAME: &str =
    concatcp!(SF_CONTENT_PREFIX, "bridgeCounterWindow");
