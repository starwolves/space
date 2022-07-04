use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};

use crate::core::{
    entity::spawn::SpawnEvent,
    space_plugin::plugin::{PostUpdateLabels, SummoningLabels},
};
use bevy::app::CoreStage::PostUpdate;

use super::{
    entity_update::gi_probe_update,
    spawn::{summon_gi_probe, summon_raw_gi_probe, GIProbeSummoner},
};

pub struct GIProbePlugin;

impl Plugin for GIProbePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(gi_probe_update),
        )
        .add_system((summon_gi_probe::<GIProbeSummoner>).after(SummoningLabels::TriggerSummon))
        .add_system((summon_raw_gi_probe).after(SummoningLabels::TriggerSummon))
        .add_event::<SpawnEvent<GIProbeSummoner>>();
    }
}
