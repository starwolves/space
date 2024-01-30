use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use entity::entity_types::register_entity_type;
use resources::ordering::{BuildingSet, PreUpdate};

use crate::spawn::{build_point_lights, PointLightType};

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (build_point_lights::<PointLightType>).in_set(BuildingSet::NormalBuild),
        );
        register_entity_type::<PointLightType>(app);
    }
}
