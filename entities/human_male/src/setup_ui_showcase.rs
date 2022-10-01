use bevy::prelude::{Added, Commands, EventWriter, Query};
use entity::spawn::{SpawnData, SpawnEvent};
use humanoid::humanoid::HUMAN_MALE_ENTITY_NAME;
use jumpsuit_security::jumpsuit::JUMPSUIT_SECURITY_ENTITY_NAME;
use pawn::pawn::{PawnDesignation, PersistentPlayerData};
use pistol_l1::pistol_l1::PISTOL_L1_ENTITY_NAME;
use player_controller::connection::{SetupPhase, SpawnPawnData};
use server::core::ConnectedPlayer;
use showcase::core::ShowcaseData;

use crate::spawn::HumanMaleSummoner;

/// Initialize the setup UI by spawning in showcase entities etc.
pub(crate) fn human_male_setup_ui(
    query: Query<(&ConnectedPlayer, &PersistentPlayerData), Added<SetupPhase>>,
    mut summon_human_male: EventWriter<SpawnEvent<HumanMaleSummoner>>,
    mut commands: Commands,
) {
    for (connected_player_component, persistent_player_data_component) in query.iter() {
        let passed_inventory_setup = vec![
            (
                "jumpsuit".to_string(),
                JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
            ),
            ("holster".to_string(), PISTOL_L1_ENTITY_NAME.to_string()),
        ];

        let human_male_entity = commands.spawn().id();

        summon_human_male.send(SpawnEvent {
            spawn_data: SpawnData {
                entity: human_male_entity,
                showcase_data_option: Some(ShowcaseData {
                    handle: connected_player_component.handle,
                }),
                entity_name: HUMAN_MALE_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            summoner: HumanMaleSummoner {
                character_name: persistent_player_data_component.character_name.clone(),
                user_name: persistent_player_data_component.user_name.clone(),
                spawn_pawn_data: SpawnPawnData {
                    persistent_player_data: persistent_player_data_component.clone(),
                    connected_player_option: Some(connected_player_component.clone()),
                    inventory_setup: passed_inventory_setup,
                    designation: PawnDesignation::Showcase,
                },
            },
        });
    }
}
