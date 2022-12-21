use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use entity::spawn::SpawnEvent;
use resources::labels::SummoningLabels;

use crate::spawn::{summon_point_light, summon_raw_point_light, PointLightSummoner};

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            (summon_point_light::<PointLightSummoner>).after(SummoningLabels::TriggerSummon),
        )
        .add_system((summon_raw_point_light).after(SummoningLabels::TriggerSummon))
        .add_event::<SpawnEvent<PointLightSummoner>>();
    }
}
