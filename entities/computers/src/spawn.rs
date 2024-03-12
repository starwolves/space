use bevy::{
    math::{Mat4, Quat, Vec3},
    prelude::{Commands, EventReader, Transform},
};
use bevy_xpbd_3d::prelude::{CoefficientCombine, Collider, Friction};
use entity::{
    entity_macros::Identity,
    entity_types::EntityType,
    examine::{Examinable, RichName},
    health::Health,
    spawn::{BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData, SpawnEntity},
};
use physics::{
    rigid_body::STANDARD_BODY_FRICTION,
    spawn::{RigidBodyBuilder, RigidBodyBundle},
};
use std::collections::BTreeMap;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

impl BaseEntityBuilder<NoData> for ComputerType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let template_examine_text = "A computer used by bridge personnel.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "bridge computer".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(ComputerType::new()),
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl RigidBodyBuilder<NoData> for ComputerType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::new(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombine::Min;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 0.7, 1.),
            collider_transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
            ..Default::default()
        }
    }
}

#[derive(Clone, Identity)]
pub struct ComputerType {
    pub identifier: String,
}
impl Default for ComputerType {
    fn default() -> Self {
        ComputerType {
            identifier: SF_CONTENT_PREFIX.to_owned() + "bridge_computer",
        }
    }
}

pub fn build_computers<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.read() {
        commands
            .entity(spawn_event.spawn_data.entity.unwrap())
            .insert(Computer);
    }
}
use resources::core::SF_CONTENT_PREFIX;

use super::computer::Computer;
