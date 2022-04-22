use std::collections::HashMap;

use bevy_ecs::system::{Commands, ResMut};
use bevy_transform::components::Transform;

use crate::space::{
    core::{
        entity::resources::{EntityDataResource, PawnDesignation, SpawnPawnData},
        gridmap::{functions::gridmap_functions::cell_id_to_world, resources::Vec3Int},
        pawn::components::PersistentPlayerData,
    },
    entities::human_male_pawn::spawn::HumanMalePawnBundle,
};

//to spawn an ai add this to the space plugin in space/mod.rs
// .add_startup_system(spawn_ai.after(StartupLabels::ServerIsLive))

pub fn spawn_ai(mut commands: Commands, entity_data: ResMut<EntityDataResource>) {
    let persistent_player_placeholder = PersistentPlayerData::default();

    let passed_inventory_setup = vec![
        ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
        ("helmet".to_string(), "helmetSecurity".to_string()),
        ("holster".to_string(), "pistolL1".to_string()),
        ("left_hand".to_string(), "constructionTool".to_string()),
    ];
    HumanMalePawnBundle::spawn(
        Transform::from_translation(cell_id_to_world(Vec3Int { x: 0, y: -1, z: 0 })),
        &mut commands,
        true,
        Some(SpawnPawnData {
            data: (
                &persistent_player_placeholder,
                None,
                passed_inventory_setup,
                PawnDesignation::Ai,
                None,
                None,
                None,
                &entity_data,
            ),
        }),
        None,
        false,
        HashMap::new(),
    );
}
