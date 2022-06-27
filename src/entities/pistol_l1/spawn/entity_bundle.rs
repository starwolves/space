use std::collections::BTreeMap;

use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        entity::{
            resources::SpawnData,
            spawn::{BaseEntityBundle, BaseEntitySummonable, NoData},
        },
        examinable::components::{Examinable, RichName},
    },
    entities::pistol_l1::PistolL1Summoner,
};

use super::PISTOL_L1_ENTITY_NAME;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.00000035355248, 0.707105, 0.7071085), 3.1415951),
        Vec3::new(0., 0.116, 0.),
    ))
}

impl BaseEntitySummonable<NoData> for PistolL1Summoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A standard issue laser pistol. It is a lethal weapon.".to_string(),
        );

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "laser pistol".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: PISTOL_L1_ENTITY_NAME.to_string(),

            ..Default::default()
        }
    }
}
