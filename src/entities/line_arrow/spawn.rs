use std::collections::{BTreeMap, HashMap};

use bevy_core::Timer;
use bevy_ecs::{entity::Entity, event::EventWriter, system::Commands};
use bevy_log::warn;
use bevy_transform::components::Transform;

use crate::core::{
    entity::{
        components::{EntityData, EntityUpdates},
        events::NetShowcase,
        resources::{SpawnHeldData, SpawnPawnData},
    },
    examinable::components::{Examinable, RichName},
    networking::resources::ConsoleCommandVariantValues,
    physics::components::{WorldMode, WorldModes},
    rigid_body::components::{CachedBroadcastTransform, DefaultTransform},
    sensable::components::Sensable,
    static_body::components::StaticTransform,
};

use super::components::{LineArrow, PointArrow};

pub struct LineArrowBundle;

impl LineArrowBundle {
    pub fn spawn(
        passed_transform: Transform,
        commands: &mut Commands,
        correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        _default_map_spawn: bool,
        properties: HashMap<String, ConsoleCommandVariantValues>,
    ) -> Entity {
        spawn_entity(
            commands,
            Some(passed_transform),
            false,
            None,
            false,
            None,
            &mut None,
            correct_transform,
            properties,
        )
    }
}

fn spawn_entity(
    commands: &mut Commands,

    passed_transform_option: Option<Transform>,

    _held: bool,
    _holder_entity_option: Option<Entity>,

    _showcase_instance: bool,
    _showcase_handle_option: Option<u32>,

    _net_showcase: &mut Option<&mut EventWriter<NetShowcase>>,

    correct_transform: bool,
    properties: HashMap<String, ConsoleCommandVariantValues>,
) -> Entity {
    let mut this_transform;
    let default_transform = Transform::identity();

    match passed_transform_option {
        Some(transform) => {
            this_transform = transform;
        }
        None => {
            this_transform = default_transform;
        }
    }

    if correct_transform {
        this_transform.rotation = default_transform.rotation;
    }

    let duration;

    match properties.get("duration").unwrap() {
        ConsoleCommandVariantValues::Int(dur) => {
            duration = dur;
        }
        _ => {
            warn!("invalid duration type");
            return Entity::from_bits(0);
        }
    }

    let template_examine_text = "A holographic arrow without additional data points.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);

    let mut builder = commands.spawn_bundle((
        EntityData {
            entity_class: "entity".to_string(),
            entity_name: "lineArrow".to_string(),
            ..Default::default()
        },
        EntityUpdates::default(),
        WorldMode {
            mode: WorldModes::Static,
        },
        StaticTransform {
            transform: this_transform,
        },
        CachedBroadcastTransform::default(),
        Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "arrow".to_string(),
                n: true,
                ..Default::default()
            },
            ..Default::default()
        },
        DefaultTransform {
            transform: default_transform,
        },
        LineArrow,
        PointArrow {
            timer: Timer::from_seconds(*duration as f32, false),
        },
    ));

    let entity_id = builder.id();
    builder.insert(Sensable::default());

    entity_id
}
