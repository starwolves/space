use std::collections::BTreeMap;

use super::resources::Airlock;
use bevy::{
    math::Vec3,
    prelude::{warn, Commands, EventReader, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use const_format::concatcp;
use entity::{
    entity_data::EntityGroup,
    entity_macros::Identity,
    entity_types::EntityType,
    examine::{Examinable, RichName},
    health::Health,
    spawn::{BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData, SpawnEntity},
};
use pawn::pawn::ShipAuthorizationEnum;
use physics::spawn::{RigidBodyBuilder, RigidBodyBundle};
use text_api::core::{FURTHER_ITALIC_FONT, HEALTHY_COLOR};

#[cfg(feature = "server")]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

#[cfg(feature = "server")]
impl BaseEntityBuilder<NoData> for AirlockType {
    fn get_bundle(&self, spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let description;
        let sub_name;

        if self.sub_type == SECURITY_AIRLOCK_ENTITY_NAME {
            sub_name = "security";
            description = "An air lock with ".to_string()
                + "security"
                + " department colors. It will only grant access to security personnel.";
        } else if self.sub_type == BRIDGE_AIRLOCK_ENTITY_NAME {
            sub_name = "bridge";
            description = "An air lock with ".to_string()
                + "bridge"
                + " department colors. It will only grant access to high ranked personnel.";
        } else if self.sub_type == GOVERNMENT_AIRLOCK_ENTITY_NAME {
            sub_name = "government";

            description = "An air lock with ".to_string()
                + "government"
                + " department colors. It will only grant access to a select few.";
        } else if self.sub_type == VACUUM_AIRLOCK_ENTITY_NAME {
            sub_name = "vacuum";
            description = "An air lock with ".to_string()
                + "danger markings"
                + ". On the other side is nothing but space.";
        } else {
            warn!("Unrecognized airlock sub-type {}", self.sub_type);
            sub_name = "ERR";
            description = "ERR ".to_string();
        }

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, description);
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                name: RichName {
                    name: sub_name.to_string() + " airlock",
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            entity_type: Box::new(AirlockType::new()),
            entity_group: EntityGroup::AirLock,
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            default_map_spawn: spawn_data.default_map_spawn,
        }
    }
}

#[cfg(feature = "server")]
pub const DEFAULT_AIRLOCK_Y: f32 = 1.;

#[cfg(feature = "server")]
impl RigidBodyBuilder<NoData> for AirlockType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 1., 0.2),
            collider_transform: Transform::from_translation(Vec3::new(0., DEFAULT_AIRLOCK_Y, 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
        }
    }
}

#[cfg(feature = "server")]
#[derive(Clone, Identity)]
pub struct AirlockType {
    pub identifier: String,
    pub sub_type: String,
}
impl Default for AirlockType {
    fn default() -> Self {
        Self {
            identifier: SF_CONTENT_PREFIX.to_owned() + "airLock",
            sub_type: VACUUM_AIRLOCK_ENTITY_NAME.to_owned(),
        }
    }
}

#[cfg(feature = "server")]
pub fn build_airlocks<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut airlock_spawns: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in airlock_spawns.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Airlock {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            });
    }
}
use resources::content::SF_CONTENT_PREFIX;

pub const SECURITY_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "securityAirLock1");
pub const BRIDGE_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "bridgeAirLock");
pub const GOVERNMENT_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "governmentAirLock");
pub const VACUUM_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "vacuumAirLock");
