use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;
use std::collections::BTreeMap;

use crate::core::{
    entity::{
        resources::SpawnData,
        spawn::{BaseEntityBundle, BaseEntitySummonable, NoEntityData},
    },
    examinable::components::{Examinable, RichName},
};

use super::HelmetSummoner;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

impl BaseEntitySummonable<NoEntityData> for HelmetSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoEntityData) -> BaseEntityBundle {
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
            entity_name: "helmetSecurity".to_string(),
            ..Default::default()
        }
    }
}
