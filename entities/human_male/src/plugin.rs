use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemSet};
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::SpawnEvent,
};
use humanoid::humanoid::{HUMAN_DUMMY_ENTITY_NAME, HUMAN_MALE_ENTITY_NAME};

use networking::messages::net_system;
use rigid_body::spawn::summon_rigid_body;
use server_instance::labels::{CombatLabels, PostUpdateLabels, StartupLabels, SummoningLabels};

use crate::{
    boarding::{on_spawning, NetOnSpawning},
    hands_attack_handler::hands_attack_handler,
    setup_ui_showcase::human_male_setup_ui,
    spawn::{default_human_dummy, summon_base_human_male, summon_human_male, HumanMaleSummoner},
};
use bevy::app::CoreStage::PostUpdate;
pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "server") {
            app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
                .add_system(
                    summon_human_male::<HumanMaleSummoner>
                        .before(SummoningLabels::TriggerSummon)
                        .label(SummoningLabels::NormalSummon),
                )
                .add_system_to_stage(PostUpdate, on_spawning.after(PostUpdateLabels::Net))
                .add_system(
                    (summon_base_human_male::<HumanMaleSummoner>)
                        .after(SummoningLabels::TriggerSummon),
                )
                .add_system(
                    (default_human_dummy)
                        .label(SummoningLabels::DefaultSummon)
                        .after(SummoningLabels::NormalSummon),
                )
                .add_event::<SpawnEvent<HumanMaleSummoner>>()
                .add_system(
                    (summon_rigid_body::<HumanMaleSummoner>).after(SummoningLabels::TriggerSummon),
                )
                .add_system(
                    hands_attack_handler
                        .label(CombatLabels::WeaponHandler)
                        .after(CombatLabels::CacheAttack),
                )
                .add_system(human_male_setup_ui.label(SummoningLabels::TriggerSummon))
                .add_event::<NetOnSpawning>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetOnSpawning>),
                );
        }
    }
}

#[cfg(feature = "server")]
pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: HUMAN_DUMMY_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: HUMAN_MALE_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
