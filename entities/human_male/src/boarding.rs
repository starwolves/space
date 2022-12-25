use bevy::prelude::{Added, Commands, Entity, EventWriter, Query, ResMut};

use networking::server::{ConnectedPlayer, HandleToEntity};
use player::names::UsedNames;

use bevy::prelude::warn;
use bevy::prelude::Res;
use pawn::pawn::Spawning;
use pawn::pawn::{PawnDesignation, SpawnPawnData};
use player::account::Accounts;

use entity::spawn::EntityBuildData;
use setup_ui::core::SetupUiUserDataSets;

use entity::spawn::SpawnEntity;

use crate::spawn::HumanMaleType;
use pawn::pawn::Pawn;
use player::boarding::PlayerBoarded;
/// Spawn player as human male with preset inventory.
#[cfg(feature = "server")]
pub(crate) fn spawn_boarding_player(
    query: Query<(Entity, &Spawning, &ConnectedPlayer), Added<Spawning>>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut used_names: ResMut<UsedNames>,
    mut builder_human_male: EventWriter<SpawnEntity<HumanMaleType>>,
    accounts: Res<Accounts>,
    setup_ui_datas: Res<SetupUiUserDataSets>,
    mut boarded: EventWriter<PlayerBoarded>,
) {
    use construction_tool_admin::spawn::ConstructionToolType;
    use entity::entity_types::EntityType;
    use helmet_security::spawn::HelmetType;
    use jumpsuit_security::spawn::JumpsuitType;
    use pistol_l1::spawn::PistolL1Type;

    for (entity_id, spawning_component, connected_player_component) in query.iter() {
        let passed_inventory_setup: Vec<(String, Box<dyn EntityType>)> = vec![
            ("jumpsuit".to_string(), Box::new(JumpsuitType::default())),
            ("helmet".to_string(), Box::new(HelmetType::default())),
            ("holster".to_string(), Box::new(PistolL1Type::default())),
            (
                "left_hand".to_string(),
                Box::new(ConstructionToolType::default()),
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

        builder_human_male.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity: new_entity,
                entity_transform: spawning_component.transform.clone(),
                ..Default::default()
            },
            builder: HumanMaleType {
                spawn_pawn_data: SpawnPawnData {
                    pawn_component: Pawn {
                        character_name: setup_data.character_name.clone(),
                        ..Default::default()
                    },
                    connected_player_option: Some(connected_player_component.clone()),
                    designation: PawnDesignation::Player,
                },
                identifier: HumanMaleType::default().to_string(),
            },
        });

        let handle;

        match handle_to_entity.inv_map.get(&entity_id) {
            Some(h) => {
                handle = *h;
            }
            None => {
                continue;
            }
        }

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
                boarded.send(PlayerBoarded {
                    handle,
                    entity: new_entity,
                    character_name: setup_data.character_name.clone(),
                    account_name: n.to_string(),
                });
            }
            None => {
                warn!("Couldn't find account name of {}", handle);
            }
        }
    }
}
