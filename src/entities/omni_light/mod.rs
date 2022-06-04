use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};

pub mod components;
pub mod entity_update;
pub mod process_content;
pub mod spawn;

use bevy_app::CoreStage::PostUpdate;

use crate::core::{entity::spawn::SpawnEvent, PostUpdateLabels, SummoningLabels};

use self::{
    entity_update::omni_light_update,
    spawn::{summon_omni_light, summon_raw_omni_light, OmniLightSummoner},
};

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
