use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, ResMut},
};
use bevy_transform::prelude::Transform;
use std::collections::{BTreeMap, HashMap};

use crate::core::{
    connected_player::functions::name_generator::get_dummy_name,
    entity::{
        events::NetShowcase,
        resources::{PawnDesignation, SpawnData},
        spawn::{
            base_entity_builder, BaseEntityBundle, BaseEntityData, BaseEntitySummonable, SpawnEvent,
        },
    },
    examinable::components::{Examinable, RichName},
    networking::resources::ReliableServerMessage,
    pawn::resources::UsedNames,
};

use super::{HumanMaleSummoner, HUMAN_MALE_ENTITY_NAME};

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

pub struct HumanMaleSummonData {
    pub used_names: UsedNames,
}

impl BaseEntitySummonable<HumanMaleSummonData> for HumanMaleSummoner {
    fn get_bundle(
        &self,
        _spawn_data: &SpawnData,
        mut entity_data: HumanMaleSummonData,
    ) -> BaseEntityBundle {
        let character_name;

        match self.spawn_pawn_data.designation {
            PawnDesignation::Dummy => {
                character_name = get_dummy_name(&mut entity_data.used_names);
            }
            PawnDesignation::Ai => {
                character_name = "Ai".to_string();
            }
            _ => {
                character_name = self
                    .spawn_pawn_data
                    .persistent_player_data
                    .character_name
                    .clone();
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
            entity_name: HUMAN_MALE_ENTITY_NAME.to_string(),
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
    mut net_showcase: EventWriter<NetShowcase>,
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
                entity_type: base_entity_bundle.entity_name.clone(),
                examinable: base_entity_bundle.examinable,
                health: base_entity_bundle.health,
                entity_group: base_entity_bundle.entity_group,
                tab_actions_option: base_entity_bundle.tab_actions_option,
                default_map_spawn: base_entity_bundle.default_map_spawn,
                showcase_handle_option: spawn_event.spawn_data.showcase_data_option.clone(),
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
        );

        match &spawn_event.spawn_data.showcase_data_option {
            Some(showcase_data) => {
                net_showcase.send(NetShowcase {
                    handle: showcase_data.handle,
                    message: ReliableServerMessage::LoadEntity(
                        "entity".to_string(),
                        base_entity_bundle.entity_name,
                        HashMap::new(),
                        spawn_event.spawn_data.entity.to_bits(),
                        true,
                        "main".to_string(),
                        "ColorRect/background/VBoxContainer/HBoxContainer/3dviewportPopup/Control/TabContainer/3D Viewport/Control/ViewportContainer/Viewport/Spatial".to_string(),
                        false,
                    ),
                });
            }
            None => {}
        }
    }
}
