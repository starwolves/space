use bevy::prelude::{Added, Commands, EventWriter, Query};
use entity::showcase::ShowcaseData;
use entity::spawn::{EntityBuildData, SpawnEntity};

use crate::spawn::HumanMaleType;
use entity::entity_types::EntityType;
use jumpsuit_security::spawn::JumpsuitType;
use networking::server::ConnectedPlayer;
use pawn::pawn::Pawn;
use pawn::pawn::PawnDesignation;
use pawn::pawn::SpawnPawnData;
use pistol_l1::spawn::PistolL1Type;
use player::connections::SetupPhase;

/// Initialize the setup UI by spawning in showcase entities etc.
#[cfg(feature = "server")]
pub(crate) fn human_male_setup_ui(
    query: Query<(&ConnectedPlayer, &Pawn), Added<SetupPhase>>,
    mut spawn_human_male: EventWriter<SpawnEntity<HumanMaleType>>,
    mut commands: Commands,
) {
    for (connected_player_component, persistent_player_data_component) in query.iter() {
        let passed_inventory_setup: Vec<(String, Box<dyn EntityType>)> = vec![
            ("jumpsuit".to_string(), Box::new(JumpsuitType::default())),
            ("holster".to_string(), Box::new(PistolL1Type::default())),
        ];

        let human_male_entity = commands.spawn(()).id();

        spawn_human_male.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity: human_male_entity,
                showcase_data_option: Some(ShowcaseData {
                    handle: connected_player_component.handle,
                }),
                ..Default::default()
            },
            builder: HumanMaleType {
                spawn_pawn_data: SpawnPawnData {
                    pawn_component: persistent_player_data_component.clone(),
                    connected_player_option: Some(connected_player_component.clone()),
                    designation: PawnDesignation::Showcase,
                },
                identifier: HumanMaleType::default().to_string(),
            },
        });
    }
}
