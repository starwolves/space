use std::collections::BTreeMap;

use bevy::math::Mat4;
use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::Transform;
use bevy_xpbd_3d::prelude::CoefficientCombine;
use bevy_xpbd_3d::prelude::Collider;
use bevy_xpbd_3d::prelude::Friction;
use entity::entity_macros::Identity;
use entity::entity_types::EntityType;
use entity::examine::Examinable;
use entity::examine::RichName;
use entity::spawn::BaseEntityBuilder;
use entity::spawn::BaseEntityBundle;
use entity::spawn::EntityBuildData;
use entity::spawn::NoData;
use entity::spawn::SpawnEntity;
use physics::rigid_body::STANDARD_BODY_FRICTION;
use physics::spawn::RigidBodyBuilder;
use physics::spawn::RigidBodyBundle;
use resources::content::SF_CONTENT_PREFIX;

use super::ball::Ball;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

impl BaseEntityBuilder<NoData> for BallType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, "A ball.".to_string());
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "ball".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(BallType::new()),
            ..Default::default()
        }
    }
}

impl RigidBodyBuilder<NoData> for BallType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::new(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombine::Multiply;

        RigidBodyBundle {
            collider: Collider::ball(0.5),
            collider_transform: Transform::IDENTITY,
            collider_friction: friction,
            ..Default::default()
        }
    }
}

#[derive(Clone, Identity)]
pub struct BallType {
    pub identifier: String,
}
impl Default for BallType {
    fn default() -> Self {
        Self {
            identifier: SF_CONTENT_PREFIX.to_string() + "ball",
        }
    }
}

pub fn build_balls<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands.entity(spawn_event.spawn_data.entity).insert(Ball);
    }
}
