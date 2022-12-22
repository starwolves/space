use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use entity::spawn::SpawnEntity;
use resources::labels::BuildingLabels;

use crate::spawn::{build_point_lights, build_raw_point_lights, PointLightBuilder};

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            (build_point_lights::<PointLightBuilder>).after(BuildingLabels::TriggerBuild),
        )
        .add_system((build_raw_point_lights).after(BuildingLabels::TriggerBuild))
        .add_event::<SpawnEntity<PointLightBuilder>>();
    }
}
