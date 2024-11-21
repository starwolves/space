use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, EventReader, Transform};
use bevy_xpbd_3d::prelude::{CoefficientCombine, Collider, Friction};
use entity::entity_macros::Identity;
use entity::entity_types::EntityType;
use entity::examine::{Examinable, RichName};
use entity::spawn::{BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData, SpawnEntity};
use physics::rigid_body::STANDARD_BODY_FRICTION;
use physics::spawn::{RigidBodyBuilder, RigidBodyBundle};
use resources::core::SF_CONTENT_PREFIX;
use std::collections::BTreeMap;

pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

impl BaseEntityBuilder<NoData> for TransportShuttleType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A transportation shuttle that can travel through space.".to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "transportation shuttle".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(TransportShuttleType::new()),
            ..Default::default()
        }
    }
}
impl RigidBodyBuilder<NoData> for TransportShuttleType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::new(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombine::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,
            gravity_scale: 0.,
            ..Default::default()
        }
    }
}

#[derive(Clone, Identity)]
pub struct TransportShuttleType {
    pub identifier: String,
}
#[derive(Component)]
pub struct TransportShuttle;

impl Default for TransportShuttleType {
    fn default() -> Self {
        TransportShuttleType {
            identifier: SF_CONTENT_PREFIX.to_string() + "transport_shuttle",
        }
    }
}

pub fn build_transport_shuttle<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.read() {
        commands
            .entity(spawn_event.spawn_data.entity.unwrap())
            .insert(TransportShuttle);
    }
}
