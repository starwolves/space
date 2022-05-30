use bevy_transform::prelude::Transform;
use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Commands};

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
    physics::components::{WorldMode, WorldModes},
    rigid_body::components::{CachedBroadcastTransform, RigidBodyDisabled},
    sensable::components::Sensable,
    tab_actions::components::TabActions,
};

use super::components::EntityGroup;

pub struct EntityBundle {
    pub default_transform: Transform,
    pub examinable: Examinable,
    pub entity_name: String,
    pub health: Health,
    pub entity_group: EntityGroup,
    pub tab_actions_option: Option<TabActions>,
    pub default_map_spawn: bool,
}

impl Default for EntityBundle {
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
    pub dynamicbody: bool,
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
            dynamicbody: true,
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

pub fn base_entity_builder(commands: &mut Commands, entity: Entity, data: BaseEntityData) {
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

    match data.is_item_in_storage {
        true => {
            builder.insert_bundle((
                RigidBodyDisabled,
                WorldMode {
                    mode: WorldModes::Worn,
                },
            ));
        }
        false => match data.dynamicbody {
            true => {
                builder.insert(WorldMode {
                    mode: WorldModes::Physics,
                });
            }
            false => {}
        },
    }
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
) {
    let mut builder = commands.entity(entity);

    match showcase_data_option {
        Some(handle) => {
            builder.insert(Showcase {
                handle: handle.handle,
            });
            handle.event_writer.send(NetShowcase {
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
