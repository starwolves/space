use std::collections::BTreeMap;

use bevy::math::Mat4;
use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Transform;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use combat::attack::DEFAULT_INVENTORY_ITEM_DAMAGE;
use entity::entity_data::RawSpawnEvent;
use entity::entity_types::BoxedEntityType;
use entity::entity_types::EntityType;
use entity::examine::Examinable;
use entity::examine::RichName;
use entity::health::DamageFlag;
use entity::spawn::BaseEntityBuilder;
use entity::spawn::BaseEntityBundle;
use entity::spawn::DefaultSpawnEvent;
use entity::spawn::EntityBuildData;
use entity::spawn::NoData;
use entity::spawn::SpawnEntity;
use inventory::combat::DamageModel;
use inventory::combat::MeleeCombat;
use inventory::inventory::SlotType;
use inventory::item::InventoryItem;
use inventory::spawn_item::InventoryItemBuilder;
use inventory::spawn_item::InventoryItemBundle;
use physics::rigid_body::STANDARD_BODY_FRICTION;
use physics::spawn::RigidBodyBuilder;
use physics::spawn::RigidBodyBundle;
use resources::content::SF_CONTENT_PREFIX;

use super::helmet::Helmet;

#[cfg(feature = "server")]
pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

#[cfg(feature = "server")]
impl BaseEntityBuilder<NoData> for HelmetType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A standard issue helmet used by Security Officers.".to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "security helmet".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(HelmetType::new()),
            ..Default::default()
        }
    }
}
use std::collections::HashMap;

#[cfg(feature = "server")]
impl InventoryItemBuilder for HelmetType {
    fn get_bundle(&self, spawn_data: &EntityBuildData) -> InventoryItemBundle {
        let mut attachment_transforms = HashMap::new();

        attachment_transforms.insert(
            "left_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(1., 0., 0.), 3.111607897),
                Vec3::new(0., -0.003, -0.108),
            )),
        );

        let right_hand_rotation = Vec3::new(0.11473795, 0.775676679, 0.);
        let right_hand_rotation_length = right_hand_rotation.length();

        attachment_transforms.insert(
            "right_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(
                    Vec3::new(0.11473795, 0.775676679, 0.).normalize(),
                    right_hand_rotation_length,
                ),
                Vec3::new(0.064, -0.019, 0.065),
            )),
        );

        attachment_transforms.insert(
            "helmet".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(1., 0., 0.), -1.41617761),
                Vec3::new(0., 0.132, 0.05),
            )),
        );

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        InventoryItemBundle {
            inventory_item: InventoryItem {
                in_inventory_of_entity: spawn_data.holder_entity_option,
                attachment_transforms: attachment_transforms,
                drop_transform: get_default_transform(),
                slot_type: SlotType::Helmet,
                throw_force_factor: 2.,
                ..Default::default()
            },
            melee_combat: MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: DEFAULT_INVENTORY_ITEM_DAMAGE,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile_combat_option: None,
        }
    }
}
#[cfg(feature = "server")]
impl RigidBodyBuilder<NoData> for HelmetType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.208, 0.277, 0.213),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct HelmetType {
    pub identifier: String,
}
impl Default for HelmetType {
    fn default() -> Self {
        Self {
            identifier: SF_CONTENT_PREFIX.to_string() + "helmetSecurity",
        }
    }
}
impl EntityType for HelmetType {
    fn to_string(&self) -> String {
        self.identifier.clone()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        HelmetType::default()
    }
    fn is_type(&self, other_type: BoxedEntityType) -> bool {
        other_type.to_string() == self.identifier
    }
}

#[cfg(feature = "server")]
pub fn build_helmets<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Helmet);
    }
}

#[cfg(feature = "server")]
pub fn build_raw_helmets(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut builder_computer: EventWriter<SpawnEntity<HelmetType>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != HelmetType::default().to_string() {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;
        builder_computer.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_type: Box::new(HelmetType::default()),
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            builder: HelmetType::default(),
        });
    }
}

#[cfg(feature = "server")]
pub fn default_build_helmets_security(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEntity<HelmetType>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event
            .spawn_data
            .entity_type
            .is_type(Box::new(HelmetType::default()))
        {
            continue;
        }
        spawner.send(SpawnEntity {
            spawn_data: spawn_event.spawn_data.clone(),
            builder: HelmetType::default(),
        });
    }
}
