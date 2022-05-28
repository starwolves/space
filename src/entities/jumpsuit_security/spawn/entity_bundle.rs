use std::collections::BTreeMap;

use bevy_math::{Quat, Vec3};

use crate::core::{
    entity::spawn::EntityBundle,
    examinable::components::{Examinable, RichName},
};

pub fn entity_bundle() -> EntityBundle {
    let mut examine_map = BTreeMap::new();
    examine_map.insert(
        0,
        "A standard issue security jumpsuit used by Security Officers.".to_string(),
    );

    EntityBundle {
        default_rotation: Quat::from_axis_angle(
            Vec3::new(-0.00000035355248, 0.707105, 0.7071085),
            3.1415951,
        ),
        examinable: Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "security jumpsuit".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        entity_name: "jumpsuitSecurity".to_string(),
    }
}
