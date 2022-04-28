use bevy_app::{EventReader, EventWriter};
use bevy_ecs::{
    entity::Entity,
    system::{Query, Res},
};

use crate::{
    core::{
        connected_player::events::{InputBuildGraphics, NetSendWorldEnvironment},
        entity::{
            components::{EntityData, EntityUpdates},
            events::NetLoadEntity,
            functions::load_entity_for_player::load_entity,
        },
        networking::resources::{ReliableServerMessage, ServerConfigMessage},
        static_body::components::StaticTransform,
        world_environment::resources::WorldEnvironment,
    },
    entities::{gi_probe::components::GIProbe, reflection_probe::components::ReflectionProbe},
};

pub fn build_graphics_event(
    mut build_graphics_events: EventReader<InputBuildGraphics>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
    mut net_send_world_environment: EventWriter<NetSendWorldEnvironment>,
    world_environment: Res<WorldEnvironment>,
    reflection_probe_query: Query<(
        Entity,
        &ReflectionProbe,
        &StaticTransform,
        &EntityData,
        &EntityUpdates,
    )>,
    gi_probe_query: Query<(
        Entity,
        &GIProbe,
        &StaticTransform,
        &EntityData,
        &EntityUpdates,
    )>,
) {
    for build_graphics_event in build_graphics_events.iter() {
        net_send_world_environment.send(NetSendWorldEnvironment {
            handle: build_graphics_event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::WorldEnvironment(
                *world_environment,
            )),
        });

        for (
            entity,
            _gi_probe_component,
            static_transform_component,
            entity_data_component,
            entity_updates_component,
        ) in gi_probe_query.iter()
        {
            load_entity(
                &entity_updates_component.updates,
                static_transform_component.transform,
                false,
                &mut net_load_entity,
                build_graphics_event.handle,
                entity_data_component,
                entity_updates_component,
                entity,
                true,
            );
        }

        for (
            entity,
            _reflection_probe_component,
            static_transform_component,
            entity_data_component,
            entity_updates_component,
        ) in reflection_probe_query.iter()
        {
            load_entity(
                &entity_updates_component.updates,
                static_transform_component.transform,
                false,
                &mut net_load_entity,
                build_graphics_event.handle,
                entity_data_component,
                entity_updates_component,
                entity,
                true,
            );
        }
    }
}
