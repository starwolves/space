use bevy::prelude::{Entity, EventReader, EventWriter, Query, info};

use crate::space_core::{components::{entity_data::EntityData, entity_updates::EntityUpdates, gi_probe::GIProbe, reflection_probe::ReflectionProbe, static_transform::StaticTransform}, events::{general::build_graphics::BuildGraphics, net::net_load_entity::NetLoadEntity}, functions::load_entity_for_player::load_entity};


pub fn build_graphics_event(
    mut build_graphics_events: EventReader<BuildGraphics>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
    reflection_probe_query : Query<(
        Entity,
        &ReflectionProbe,
        &StaticTransform,
        &EntityData,
        &EntityUpdates
    )>,
    gi_probe_query : Query<(
        Entity,
        &GIProbe,
        &StaticTransform,
        &EntityData,
        &EntityUpdates
    )>
) {

    for build_graphics_event in build_graphics_events.iter() {

        for (
            entity,
            _gi_probe_component,
            static_transform_component,
            entity_data_component,
            entity_updates_component
        ) in gi_probe_query.iter() {
            load_entity(
                &entity_updates_component.updates,
                static_transform_component.transform,
                false,
                &mut net_load_entity,
                build_graphics_event.handle,
                entity_data_component,
                entity.id()
            );
        }

        for (
            entity,
            _reflection_probe_component,
            static_transform_component,
            entity_data_component,
            entity_updates_component
        ) in reflection_probe_query.iter() {
            load_entity(
                &entity_updates_component.updates,
                static_transform_component.transform,
                false,
                &mut net_load_entity,
                build_graphics_event.handle,
                entity_data_component,
                entity.id()
            );
        }

    }

}
