use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use entity::entity_types::register_entity_type;
use resources::sets::{BuildingSet, MainSet};

use crate::spawn::{build_point_lights, PointLightType};

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (build_point_lights::<PointLightType>)
                .after(BuildingSet::TriggerBuild)
                .in_set(MainSet::Update),
        );
        register_entity_type::<PointLightType>(app);
    }
}
