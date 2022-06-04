use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};

use crate::core::entity::spawn::SpawnEvent;
use crate::core::inventory_item::entity_update::inventory_item_update;
use crate::core::{PostUpdateLabels, SummoningLabels};

use self::entity_update::reflection_probe_update;
use self::spawn::{spawn_raw_reflection_probe, summon_reflection_probe, ReflectionProbeSummoner};

pub mod components;
pub mod entity_update;
pub mod process_content;
pub mod spawn;

pub struct ReflectionProbePlugin;
impl Plugin for ReflectionProbePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(reflection_probe_update)
                .with_system(inventory_item_update),
        )
        .add_system(
            (summon_reflection_probe::<ReflectionProbeSummoner>)
                .after(SummoningLabels::TriggerSummon),
        )
        .add_system((spawn_raw_reflection_probe).after(SummoningLabels::TriggerSummon))
        .add_event::<SpawnEvent<ReflectionProbeSummoner>>();
    }
}
