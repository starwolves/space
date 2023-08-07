use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use entity::{entity_types::register_entity_type, spawn::SpawnItemSet};
use resources::sets::MainSet;

use crate::spawn::{build_point_lights, PointLightType};

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (build_point_lights::<PointLightType>)
                .after(SpawnItemSet::SpawnHeldItem)
                .in_set(MainSet::Update),
        );
        register_entity_type::<PointLightType>(app);
    }
}
