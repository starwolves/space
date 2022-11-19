use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};

use bevy::app::CoreStage::PostUpdate;
use entity::spawn::SpawnEvent;
use resources::labels::{PostUpdateLabels, SummoningLabels};

use super::{
    entity_update::gi_probe_update,
    spawn::{summon_gi_probe, summon_raw_gi_probe, GIProbeSummoner},
};

pub struct GIProbePlugin;

impl Plugin for GIProbePlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
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
}
