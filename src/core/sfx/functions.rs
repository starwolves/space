use bevy_ecs::system::{Commands, EntityCommands};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    rigid_body::components::{CachedBroadcastTransform, UpdateTransform},
    sensable::components::Sensable,
};

pub fn repeating_sfx_builder<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> + Sync + Send>,
) -> EntityCommands<'w, 's, 'a> {
    let commands = commands.spawn_bundle((
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
    (builder)(commands)
}

pub fn sfx_builder<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> + Sync + Send>,
) -> EntityCommands<'w, 's, 'a> {
    let commands = commands.spawn_bundle((
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
    (builder)(commands)
}

pub fn ambience_sfx_builder<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    rigid_body_position: Transform,
    builder: Box<dyn Fn(EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> + Sync + Send>,
) -> EntityCommands<'w, 's, 'a> {
    let commands = commands.spawn_bundle((
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
    (builder)(commands)
}
