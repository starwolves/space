use bevy_transform::prelude::Transform;
use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::Commands,
};

use crate::core::{
    connected_player::systems::on_setupui::ENTITY_SPAWN_PARENT,
    entity::{
        components::{EntityData, EntityUpdates, Showcase},
        events::NetShowcase,
        resources::ShowcaseData,
    },
    examinable::components::Examinable,
    health::components::Health,
    networking::resources::{EntityUpdateData, ReliableServerMessage},
    rigid_body::components::CachedBroadcastTransform,
    sensable::components::Sensable,
    tab_actions::components::TabActions,
};

use super::{components::EntityGroup, resources::SpawnData};

pub struct BaseEntityBundle {
    pub default_transform: Transform,
    pub examinable: Examinable,
    pub entity_name: String,
    pub health: Health,
    pub entity_group: EntityGroup,
    pub tab_actions_option: Option<TabActions>,
    pub default_map_spawn: bool,
}

impl Default for BaseEntityBundle {
    fn default() -> Self {
        Self {
            entity_group: EntityGroup::None,
            default_transform: Transform::default(),
            examinable: Examinable::default(),
            entity_name: "".to_string(),
            health: Health::default(),
            tab_actions_option: None,
            default_map_spawn: false,
        }
    }
}

pub struct BaseEntityData {
    pub entity_type: String,
    pub examinable: Examinable,
    pub sensable: Sensable,
    pub health: Health,
    pub is_item_in_storage: bool,
    pub entity_group: EntityGroup,
    pub tab_actions_option: Option<TabActions>,
    pub default_map_spawn: bool,
    pub is_showcase: bool,
}

impl Default for BaseEntityData {
    fn default() -> Self {
        Self {
            entity_group: EntityGroup::None,
            entity_type: "".to_string(),
            examinable: Examinable::default(),
            sensable: Sensable::default(),
            health: Health::default(),
            is_item_in_storage: false,
            tab_actions_option: None,
            default_map_spawn: false,
            is_showcase: false,
        }
    }
}

pub fn base_entity_builder(commands: &mut Commands, data: BaseEntityData, entity: Entity) {
    let mut builder = commands.entity(entity);
    builder.insert_bundle((
        EntityData {
            entity_class: "entity".to_string(),
            entity_name: data.entity_type.to_string(),
            entity_group: data.entity_group,
        },
        EntityUpdates::default(),
        CachedBroadcastTransform::default(),
    ));

    if !data.is_showcase {
        builder.insert_bundle((data.sensable, data.examinable, data.health));
    }

    match data.tab_actions_option {
        Some(a) => {
            builder.insert(a);
        }
        None => {}
    }
}

pub trait BaseEntitySummonable<Y> {
    fn get_bundle(&self, spawn_data: &SpawnData, entity_data_option: Y) -> BaseEntityBundle;
}

pub struct SpawnEvent<T> {
    pub spawn_data: SpawnData,
    pub summoner: T,
}

pub struct DefaultSpawnEvent {
    pub spawn_data: SpawnData,
}

#[derive(Default)]
pub struct ShowCaseBuilderData {
    pub entity_type: String,
    pub entity_updates: HashMap<String, HashMap<String, EntityUpdateData>>,
}

pub fn showcase_builder(
    commands: &mut Commands,
    entity: Entity,
    showcase_data_option: &mut Option<ShowcaseData>,
    builder_data: ShowCaseBuilderData,
    showcase_event: &mut EventWriter<NetShowcase>,
) {
    let mut builder = commands.entity(entity);

    match showcase_data_option {
        Some(handle) => {
            builder.insert(Showcase {
                handle: handle.handle,
            });
            showcase_event.send(NetShowcase {
                handle: handle.handle,
                message: ReliableServerMessage::LoadEntity(
                    "entity".to_string(),
                    builder_data.entity_type.to_string(),
                    builder_data.entity_updates,
                    entity.to_bits(),
                    true,
                    "main".to_string(),
                    ENTITY_SPAWN_PARENT.to_string(),
                    false,
                ),
            });
        }
        None => {}
    }
}

pub fn summon_base_entity<T: BaseEntitySummonable<NoEntityData> + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let base_entity_bundle = spawn_event
            .summoner
            .get_bundle(&spawn_event.spawn_data, NoEntityData);

        base_entity_builder(
            &mut commands,
            BaseEntityData {
                entity_type: base_entity_bundle.entity_name,
                examinable: base_entity_bundle.examinable,
                health: base_entity_bundle.health,
                entity_group: base_entity_bundle.entity_group,
                tab_actions_option: base_entity_bundle.tab_actions_option,
                default_map_spawn: base_entity_bundle.default_map_spawn,
                is_item_in_storage: spawn_event.spawn_data.holder_entity_option.is_some(),
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
        );
    }
}

pub struct NoEntityData;
