use bevy_math::Quat;
use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::{
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
};

pub struct EntityBundle {
    pub default_rotation: Quat,
    pub examinable: Examinable,
    pub entity_name: String,
}

#[derive(Default)]
pub struct BaseEntityData {
    pub dynamicbody: bool,
    pub entity_type: String,
    pub examinable: Examinable,
    pub sensable: Sensable,
    pub health: Health,
    pub is_item_in_storage: bool,
}

pub fn base_entity_builder(commands: &mut Commands, entity: Entity, data: BaseEntityData) {
    let mut builder = commands.entity(entity);
    builder.insert_bundle((
        EntityData {
            entity_class: "entity".to_string(),
            entity_name: data.entity_type.to_string(),
            ..Default::default()
        },
        EntityUpdates::default(),
        CachedBroadcastTransform::default(),
        data.examinable,
        data.sensable,
        data.health,
    ));

    match data.is_item_in_storage {
        true => {
            builder.insert_bundle((
                RigidBodyDisabled,
                WorldMode {
                    mode: WorldModes::Worn,
                },
            ));
        }
        false => {
            builder.insert(WorldMode {
                mode: WorldModes::Physics,
            });
        }
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
                    "".to_string(),
                    false,
                ),
            });
        }
        None => {}
    }
}
