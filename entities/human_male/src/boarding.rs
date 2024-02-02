use bevy::{
    ecs::event::EventReader,
    prelude::{Commands, EventWriter, Query, ResMut},
};

use construction_tool::spawn::ConstructionToolType;
use entity::despawn::DespawnEntity;
use entity::entity_types::EntityType;
use networking::server::{ConnectedPlayer, HandleToEntity, OutgoingReliableServerMessage};
use player::names::UsedNames;
use player::{connections::Accounts, net::PlayerServerMessage};

use bevy::log::warn;
use bevy::prelude::Res;
use pawn::pawn::{PawnDesignation, SpawnPawnData, SpawningPlayer};

use entity::spawn::EntityBuildData;
use setup_menu::core::SetupUiUserDataSets;

use entity::spawn::SpawnEntity;

use pawn::pawn::Pawn;
use player::boarding::PlayerBoarded;

use crate::spawn::HumanMaleType;

/// Spawn player as human male with preset inventory.

pub(crate) fn spawn_boarding_player(
    query: Query<&ConnectedPlayer>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut used_names: ResMut<UsedNames>,
    mut spawn_human: EventWriter<SpawnEntity<HumanMaleType>>,
    accounts: Res<Accounts>,
    setup_ui_datas: Res<SetupUiUserDataSets>,
    mut boarded: EventWriter<PlayerBoarded>,
    mut despawn: EventWriter<DespawnEntity>,
    mut spawns: EventReader<SpawningPlayer>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
) {
    for spawning_event in spawns.read() {
        match query.get(spawning_event.entity) {
            Ok(connected_player_component) => {
                let setup_data;

                match setup_ui_datas.list.get(&connected_player_component.handle) {
                    Some(data) => {
                        setup_data = data;
                    }
                    None => {
                        warn!(
                            "Could not find setup data for {}",
                            connected_player_component.handle
                        );
                        continue;
                    }
                }

                let new_human_entity = commands.spawn(()).id();

                let passed_inventory_setup: Vec<Box<dyn EntityType>> =
                    vec![Box::new(ConstructionToolType::default())];

                spawn_human.send(SpawnEntity {
                    spawn_data: EntityBuildData {
                        entity: new_human_entity,
                        entity_transform: spawning_event.transform.clone(),
                        ..Default::default()
                    },
                    entity_type: HumanMaleType {
                        spawn_pawn_data: SpawnPawnData {
                            pawn_component: Pawn {
                                character_name: setup_data.character_name.clone(),
                                ..Default::default()
                            },
                            connected_player_option: Some(connected_player_component.clone()),
                            designation: PawnDesignation::Player,
                            inventory_setup: passed_inventory_setup,
                        },
                        ..Default::default()
                    },
                });

                let handle;

                match handle_to_entity.inv_map.get(&spawning_event.entity) {
                    Some(h) => {
                        handle = *h;
                    }
                    None => {
                        continue;
                    }
                }

                handle_to_entity.inv_map.remove(&spawning_event.entity);
                handle_to_entity.inv_map.insert(new_human_entity, handle);

                handle_to_entity.map.remove(&handle);
                handle_to_entity.map.insert(handle, new_human_entity);

                used_names
                    .names
                    .insert(setup_data.character_name.clone(), new_human_entity);

                despawn.send(DespawnEntity {
                    entity: spawning_event.entity,
                });

                server.send(OutgoingReliableServerMessage {
                    handle: handle,
                    message: PlayerServerMessage::PawnId(new_human_entity),
                });

                match accounts.list.get(&handle) {
                    Some(n) => {
                        boarded.send(PlayerBoarded {
                            handle,
                            entity: new_human_entity,
                            character_name: setup_data.character_name.clone(),
                            account_name: n.to_string(),
                        });
                    }
                    None => {
                        warn!("Couldn't find account name of {}", handle);
                    }
                }
            }
            Err(_) => {
                warn!("spawn_board couldnt find in query.");
            }
        }
    }
}
