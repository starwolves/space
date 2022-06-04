use bevy_ecs::{
    event::EventReader,
    system::{Commands, ResMut},
};
use bevy_transform::prelude::Transform;
use std::collections::BTreeMap;

use crate::core::{
    connected_player::functions::name_generator::get_dummy_name,
    entity::{
        resources::{PawnDesignation, SpawnData},
        spawn::{
            base_entity_builder, BaseEntityBundle, BaseEntityData, BaseEntitySummonable, SpawnEvent,
        },
    },
    examinable::components::{Examinable, RichName},
    pawn::resources::UsedNames,
};

use super::HumanMaleSummoner;

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

pub struct HumanMaleSummonData {
    pub used_names: UsedNames,
}

impl BaseEntitySummonable<HumanMaleSummonData> for HumanMaleSummoner {
    fn get_bundle(
        &self,
        spawn_data: &SpawnData,
        mut entity_data: HumanMaleSummonData,
    ) -> BaseEntityBundle {
        let (
            persistent_player_data_component,
            _connected_player_component,
            _passed_inventory_setup,
            pawn_designation,
            _default_user_name_option,
        ) = spawn_data.pawn_data_option.clone().unwrap().data;

        let character_name;

        match pawn_designation {
            PawnDesignation::Dummy => {
                character_name = get_dummy_name(&mut entity_data.used_names);
            }
            PawnDesignation::Ai => {
                character_name = "Ai".to_string();
            }
            _ => {
                character_name = persistent_player_data_component.character_name.clone();
            }
        }

        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A standard issue helmet used by Security Officers.".to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: character_name.clone(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: "humanMale".to_string(),
            ..Default::default()
        }
    }
}

pub fn summon_base_human_male<
    T: BaseEntitySummonable<HumanMaleSummonData> + Send + Sync + 'static,
>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
    used_names: ResMut<UsedNames>,
) {
    for spawn_event in spawn_events.iter() {
        let base_entity_bundle = spawn_event.summoner.get_bundle(
            &spawn_event.spawn_data,
            HumanMaleSummonData {
                used_names: used_names.clone(),
            },
        );

        base_entity_builder(
            &mut commands,
            BaseEntityData {
                entity_type: base_entity_bundle.entity_name,
                examinable: base_entity_bundle.examinable,
                health: base_entity_bundle.health,
                entity_group: base_entity_bundle.entity_group,
                tab_actions_option: base_entity_bundle.tab_actions_option,
                default_map_spawn: base_entity_bundle.default_map_spawn,
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
        );
    }
}
