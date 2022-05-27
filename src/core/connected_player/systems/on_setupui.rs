use std::collections::HashMap;

use bevy_ecs::{
    event::EventWriter,
    prelude::Added,
    system::{Commands, Query, Res, ResMut},
};
use bevy_transform::components::Transform;

use crate::{
    core::{
        configuration::resources::{ServerId, MOTD},
        connected_player::{
            components::{ConnectedPlayer, SetupPhase},
            events::NetOnSetupUI,
            functions::name_generator,
        },
        entity::{
            events::NetShowcase,
            resources::{
                EntityDataResource, PawnDesignation, ShowcaseData, SpawnData, SpawnPawnData,
            },
        },
        networking::resources::{EntityUpdateData, EntityWorldType, ReliableServerMessage},
        pawn::{components::PersistentPlayerData, resources::UsedNames},
    },
    entities::human_male::spawn::HumanMaleBundle,
};

pub const INPUT_NAME_PATH_FULL : &str = "setupUI::ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
pub const INPUT_NAME_PATH : &str = "ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
pub const ENTITY_SPAWN_PARENT : &str = "ColorRect/background/VBoxContainer/HBoxContainer/3dviewportPopup/Control/TabContainer/3D Viewport/Control/ViewportContainer/Viewport/Spatial";

pub fn on_setupui<'a, 'b, 'c, 'd, 'w, 's>(
    used_names: Res<UsedNames>,
    server_id: Res<ServerId>,

    query: Query<(&ConnectedPlayer, &PersistentPlayerData), Added<SetupPhase>>,
    mut net_showcase: EventWriter<'b, 'c, NetShowcase>,

    entity_data: ResMut<EntityDataResource>,

    mut net_on_setupui: EventWriter<NetOnSetupUI>,
    mut commands: Commands<'w, 's>,
    motd: Res<MOTD>,
) {
    for (connected_player_component, persistent_player_data_component) in query.iter() {
        let suggested_name = name_generator::get_full_name(true, true, &used_names);

        let mut hash_map_data = HashMap::new();

        hash_map_data.insert(
            "label_text".to_string(),
            EntityUpdateData::String(suggested_name),
        );

        let mut hash_map_path = HashMap::new();

        hash_map_path.insert(INPUT_NAME_PATH_FULL.to_string(), hash_map_data);

        net_on_setupui.send(NetOnSetupUI {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::EntityUpdate(
                server_id.id.to_bits(),
                hash_map_path,
                false,
                EntityWorldType::Main,
            ),
        });

        net_on_setupui.send(NetOnSetupUI {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ChatMessage(motd.message.clone()),
        });

        let passed_inventory_setup = vec![
            ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
            ("holster".to_string(), "pistolL1".to_string()),
        ];

        HumanMaleBundle::spawn(SpawnData {
            entity_transform: Transform::identity(),
            commands: &mut commands,

            correct_transform: true,
            pawn_data_option: Some(SpawnPawnData {
                data: (
                    persistent_player_data_component,
                    Some(connected_player_component),
                    passed_inventory_setup,
                    PawnDesignation::Showcase,
                    None,
                    None,
                    &entity_data,
                ),
            }),
            held_data_option: None,
            default_map_spawn: false,
            properties: HashMap::new(),
            showcase_data_option: &mut Some(ShowcaseData {
                handle: connected_player_component.handle,
                event_writer: &mut net_showcase,
            }),
        });
    }
}
