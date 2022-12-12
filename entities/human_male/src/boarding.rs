use bevy::prelude::{Added, Commands, Entity, EventWriter, Query, ResMut};

use bevy::prelude::info;
use networking::server::OutgoingReliableServerMessage;
use networking::server::{ConnectedPlayer, HandleToEntity};
use player::names::UsedNames;

use bevy::prelude::warn;
use bevy::prelude::Res;
use pawn::pawn::Spawning;
use pawn::pawn::{PawnDesignation, SpawnPawnData};
use player::account::Accounts;
use player::connections::PlayerServerMessage;

use construction_tool_admin::construction_tool::CONSTRUCTION_TOOL_ENTITY_NAME;
use entity::spawn::SpawnData;
use helmet_security::helmet::HELMET_SECURITY_ENTITY_NAME;
use jumpsuit_security::jumpsuit::JUMPSUIT_SECURITY_ENTITY_NAME;
use pistol_l1::pistol_l1::PISTOL_L1_ENTITY_NAME;
use setup_ui::core::SetupUiDatas;

use entity::spawn::SpawnEvent;
use humanoid::humanoid::HUMAN_MALE_ENTITY_NAME;

use crate::spawn::HumanMaleSummoner;
/// Spawn player as human male with preset inventory.
#[cfg(feature = "server")]
pub(crate) fn on_spawning(
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    query: Query<(Entity, &Spawning, &ConnectedPlayer), Added<Spawning>>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut used_names: ResMut<UsedNames>,
    mut summon_human_male: EventWriter<SpawnEvent<HumanMaleSummoner>>,
    accounts: Res<Accounts>,
    setup_ui_datas: Res<SetupUiDatas>,
) {
    use pawn::pawn::Pawn;

    for (entity_id, spawning_component, connected_player_component) in query.iter() {
        let passed_inventory_setup = vec![
            (
                "jumpsuit".to_string(),
                JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
            ),
            (
                "helmet".to_string(),
                HELMET_SECURITY_ENTITY_NAME.to_string(),
            ),
            ("holster".to_string(), PISTOL_L1_ENTITY_NAME.to_string()),
            (
                "left_hand".to_string(),
                CONSTRUCTION_TOOL_ENTITY_NAME.to_string(),
            ),
        ];

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

        let new_entity = commands.spawn(()).id();

        summon_human_male.send(SpawnEvent {
            spawn_data: SpawnData {
                entity: new_entity,
                entity_transform: spawning_component.transform,
                entity_name: HUMAN_MALE_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            summoner: HumanMaleSummoner {
                spawn_pawn_data: SpawnPawnData {
                    pawn_component: Pawn {
                        character_name: setup_data.character_name.clone(),
                        ..Default::default()
                    },
                    connected_player_option: Some(connected_player_component.clone()),
                    inventory_setup: passed_inventory_setup,
                    designation: PawnDesignation::Player,
                },
            },
        });

        let handle = *handle_to_entity.inv_map.get(&entity_id).unwrap();

        handle_to_entity.inv_map.remove(&entity_id);
        handle_to_entity.inv_map.insert(new_entity, handle);

        handle_to_entity.map.remove(&handle);
        handle_to_entity.map.insert(handle, new_entity);

        used_names
            .names
            .insert(setup_data.character_name.clone(), new_entity);

        commands.entity(entity_id).despawn();

        match accounts.list.get(&handle) {
            Some(n) => {
                info!(
                    "{} has boarded as \"{}\". [{}][{:?}]",
                    n, setup_data.character_name, handle, new_entity
                );
            }
            None => {
                warn!("Couldn't find account name of {}", handle);
            }
        }

        server.send(OutgoingReliableServerMessage {
            handle,
            message: PlayerServerMessage::PawnId(new_entity.to_bits()),
        });
    }
}
