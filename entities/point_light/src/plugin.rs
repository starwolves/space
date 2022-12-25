use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use entity::entity_types::register_entity_type;
use resources::labels::BuildingLabels;

use crate::spawn::{build_point_lights, PointLightType};

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system((build_point_lights::<PointLightType>).after(BuildingLabels::TriggerBuild));
        register_entity_type::<PointLightType>(app);
    }
}
