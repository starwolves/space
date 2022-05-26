use bevy_ecs::{entity::Entity, system::Commands};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    rigid_body::components::{CachedBroadcastTransform, UpdateTransform},
    sensable::components::Sensable,
};

pub fn repeating_sfx_builder(
    commands: &mut Commands,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(&mut Commands) -> Entity + Sync + Send>,
) -> Entity {
    let entity = (builder)(commands);
    commands.entity(entity).insert_bundle((
        rigid_body_position,
        EntityData {
            entity_class: "RepeatingSFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        EntityUpdates::default(),
        UpdateTransform,
        CachedBroadcastTransform::default(),
    ));
    entity
}

pub fn sfx_builder(
    commands: &mut Commands,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(&mut Commands) -> Entity + Sync + Send>,
) -> Entity {
    let entity = (builder)(commands);
    commands.entity(entity).insert_bundle((
        rigid_body_position,
        EntityData {
            entity_class: "SFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        EntityUpdates::default(),
    ));
    entity
}

pub fn ambience_sfx_builder(
    commands: &mut Commands,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(&mut Commands) -> Entity + Sync + Send>,
) -> Entity {
    let entity = (builder)(commands);

    commands.entity(entity).insert_bundle((
        rigid_body_position,
        EntityData {
            entity_class: "SFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            always_sensed: true,
            ..Default::default()
        },
        EntityUpdates::default(),
    ));

    entity
}
