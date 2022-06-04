use bevy_app::{App, Plugin};

pub mod components;
pub mod entity_update;
pub mod process_content;
pub mod spawn;
use bevy_app::CoreStage::PostUpdate;
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;
use bevy_ecs::schedule::SystemSet;

use crate::core::entity::spawn::SpawnEvent;
use crate::core::{PostUpdateLabels, SummoningLabels};

use self::entity_update::gi_probe_update;
use self::spawn::{summon_gi_probe, summon_raw_gi_probe, GIProbeSummoner};

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
