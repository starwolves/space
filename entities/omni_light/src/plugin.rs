use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use entity::spawn::SpawnEvent;
use api::data::{PostUpdateLabels, SummoningLabels};

use super::entity_update::omni_light_update;
use super::spawn::{summon_omni_light, summon_raw_omni_light, OmniLightSummoner};
use bevy::app::CoreStage::PostUpdate;
pub struct OmniLightPlugin;

impl Plugin for OmniLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(omni_light_update),
        )
        .add_system((summon_omni_light::<OmniLightSummoner>).after(SummoningLabels::TriggerSummon))
        .add_system((summon_raw_omni_light).after(SummoningLabels::TriggerSummon))
        .add_event::<SpawnEvent<OmniLightSummoner>>();
    }
}
