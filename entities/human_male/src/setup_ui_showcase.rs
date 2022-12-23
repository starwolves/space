use bevy::prelude::{Added, Commands, EventWriter, Query};
use entity::showcase::ShowcaseData;
use entity::spawn::{EntityBuildData, SpawnEntity};
use humanoid::humanoid::HUMAN_MALE_ENTITY_NAME;
use jumpsuit_security::jumpsuit::JUMPSUIT_SECURITY_ENTITY_NAME;
use pistol_l1::pistol_l1::PISTOL_L1_ENTITY_NAME;

use crate::spawn::HumanMaleBuilder;
use networking::server::ConnectedPlayer;
use pawn::pawn::Pawn;
use pawn::pawn::PawnDesignation;
use pawn::pawn::SpawnPawnData;
use player::connections::SetupPhase;

/// Initialize the setup UI by spawning in showcase entities etc.
#[cfg(feature = "server")]
pub(crate) fn human_male_setup_ui(
    query: Query<(&ConnectedPlayer, &Pawn), Added<SetupPhase>>,
    mut spawn_human_male: EventWriter<SpawnEntity<HumanMaleBuilder>>,
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

        let human_male_entity = commands.spawn(()).id();

        spawn_human_male.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity: human_male_entity,
                showcase_data_option: Some(ShowcaseData {
                    handle: connected_player_component.handle,
                }),
                entity_type: HUMAN_MALE_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            builder: HumanMaleBuilder {
                spawn_pawn_data: SpawnPawnData {
                    pawn_component: persistent_player_data_component.clone(),
                    connected_player_option: Some(connected_player_component.clone()),
                    inventory_setup: passed_inventory_setup,
                    designation: PawnDesignation::Showcase,
                },
            },
        });
    }
}
