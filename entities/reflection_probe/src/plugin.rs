use api::data::{PostUpdateLabels, SummoningLabels};
use bevy::app::CoreStage::PostUpdate;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use entity::spawn::SpawnEvent;

use super::entity_update::reflection_probe_update;
use super::spawn::{spawn_raw_reflection_probe, summon_reflection_probe, ReflectionProbeSummoner};

pub struct ReflectionProbePlugin;
impl Plugin for ReflectionProbePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(reflection_probe_update),
        )
        .add_system(
            summon_reflection_probe::<ReflectionProbeSummoner>
                .after(SummoningLabels::TriggerSummon),
        )
        .add_system(spawn_raw_reflection_probe.after(SummoningLabels::TriggerSummon))
        .add_event::<SpawnEvent<ReflectionProbeSummoner>>();
    }
}
