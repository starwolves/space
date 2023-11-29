use std::collections::HashMap;

use bevy::prelude::{Component, Event, SystemSet, Transform};
use entity_macros::Identity;
use networking::server::EntityUpdateData;
use serde::{Deserialize, Serialize};

use crate::entity_types::{BoxedEntityType, EntityType};
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum EntityWorldType {
    Main,
    HealthUI,
}

use crate::init::RawEntityRon;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InterpolationSet {
    Main,
}

/// Component for entities that were included and spawned with the map itself.
#[derive(Component)]

pub struct DefaultMapEntity;

/// Event about spawning entities from ron.
#[derive(Event)]
pub struct RawSpawnEvent {
    pub raw_entity: RawEntityRon,
}
/// ron entity.
#[derive(Deserialize, Clone)]

pub struct RawEntity {
    pub entity_type: String,
    pub transform: String,
    pub data: String,
}

/// Component with the cache of the latest broadcasted transforms for its entity.
#[derive(Component, Default)]

pub struct CachedBroadcastTransform {
    pub transform: Transform,
    pub is_active: bool,
}
/// Component with transform for sound effects.
#[derive(Component)]

pub struct UpdateTransform;
/// The NodePath to the node to spawn entities in on the Godot clients.

pub const ENTITY_SPAWN_PARENT : &str = "ColorRect/background/VBoxContainer/HBoxContainer/3dviewportPopup/Control/TabContainer/3D Viewport/Control/ViewportContainer/Viewport/Spatial";

/// Check if entity updates for a player has changed. Old Godot netcode.

pub fn entity_update_changed_detection(
    changed_parameters: &mut Vec<String>,
    entity_updates: &mut HashMap<String, EntityUpdateData>,
    set: EntityUpdateData,
    parameter: String,
) {
    let get = entity_updates.get(&parameter);
    let has_changed;
    match get {
        Some(value) => {
            has_changed = !entity_data_is_matching(value, &set);
        }
        None => {
            has_changed = true;
        }
    }

    if has_changed == true {
        entity_updates.insert(parameter.clone(), set);
        changed_parameters.push(parameter);
    }
}

/// The base entity component holding base entity data.
#[derive(Component)]

pub struct EntityData {
    pub entity_type: BoxedEntityType,
    pub entity_group: EntityGroup,
}
#[derive(Clone, Identity)]
pub struct BlankEntityType {
    pub identifier: String,
}
impl Default for BlankEntityType {
    fn default() -> Self {
        Self {
            identifier: "Blank".to_string(),
        }
    }
}

#[derive(Copy, Clone, Default)]

pub enum EntityGroup {
    #[default]
    None,
    AirLock,
    CounterWindowSensor,
    Pawn,
}
/// Match entity data as a function. Old Godot netcode.

pub fn entity_data_is_matching(data1: &EntityUpdateData, data2: &EntityUpdateData) -> bool {
    let mut is_not_matching = true;

    match data1 {
        EntityUpdateData::Int(old_value) => match data2 {
            EntityUpdateData::Int(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::UInt8(old_value) => match data2 {
            EntityUpdateData::UInt8(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::String(old_value) => match data2 {
            EntityUpdateData::String(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::StringVec(old_value) => match data2 {
            EntityUpdateData::StringVec(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Float(old_value) => match data2 {
            EntityUpdateData::Float(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Transform(old_value, old_value1, old_value2) => match data2 {
            EntityUpdateData::Transform(new_value, new_value1, new_value2) => {
                is_not_matching = *new_value != *old_value
                    || *old_value1 != *new_value1
                    || *old_value2 != *new_value2;
            }
            _ => {}
        },
        EntityUpdateData::Color(r, g, b, a) => match data2 {
            EntityUpdateData::Color(r_n, g_n, b_n, a_n) => {
                is_not_matching = r != r_n && g != g_n && b != b_n && a != a_n;
            }
            _ => {}
        },
        EntityUpdateData::Bool(old_value) => match data2 {
            EntityUpdateData::Bool(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Vec3(old_value) => match data2 {
            EntityUpdateData::Vec3(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::AttachedItem(old_value0, old_value1, old_value2, old_value3) => {
            match data2 {
                EntityUpdateData::AttachedItem(new_value0, new_value1, new_value2, new_value3) => {
                    is_not_matching = *new_value0 != *old_value0
                        || *new_value1 != *old_value1
                        || *new_value2 != *old_value2
                        || *new_value3 != *old_value3;
                }
                _ => {}
            }
        }
        EntityUpdateData::WornItem(
            old_value0,
            old_value1,
            old_value2,
            old_value3,
            old_value4,
            old_value5,
        ) => match data2 {
            EntityUpdateData::WornItem(
                new_value0,
                new_value1,
                new_value2,
                new_value3,
                new_value4,
                new_value5,
            ) => {
                is_not_matching = *new_value0 != *old_value0
                    || *new_value1 != *old_value1
                    || *new_value2 != *old_value2
                    || *new_value3 != *old_value3
                    || *new_value4 != *old_value4
                    || *new_value5 != *old_value5;
            }
            _ => {}
        },
        EntityUpdateData::WornItemNotAttached(old_value0, old_value1, old_value2) => match data2 {
            EntityUpdateData::WornItemNotAttached(new_value0, new_value1, new_value2) => {
                is_not_matching = *new_value0 != *old_value0
                    || *new_value1 != *old_value1
                    || *new_value2 != *old_value2;
            }
            _ => {}
        },
        EntityUpdateData::Vec2(old_value0) => match data2 {
            EntityUpdateData::Vec2(new_value0) => is_not_matching = *new_value0 != *old_value0,
            _ => {}
        },
    }

    !is_not_matching
}

/// Get difference between this frame and last's frame entity updates per player. Old Godot netcode.

pub fn get_entity_update_difference(
    old_data: HashMap<String, HashMap<String, EntityUpdateData>>,
    new_data: &HashMap<String, HashMap<String, EntityUpdateData>>,
) -> HashMap<String, HashMap<String, EntityUpdateData>> {
    let mut difference_data = HashMap::new();

    for (new_node_path, new_data_entity_updates) in new_data {
        match old_data.get(new_node_path) {
            Some(old_data_entity_updates) => {
                for (new_entity_update_type, new_entity_update_data) in new_data_entity_updates {
                    match old_data_entity_updates.get(new_entity_update_type) {
                        Some(old_entity_update_data) => {
                            if !entity_data_is_matching(
                                new_entity_update_data,
                                old_entity_update_data,
                            ) {
                                if !difference_data.contains_key(&new_node_path.to_string()) {
                                    difference_data
                                        .insert(new_node_path.to_string(), HashMap::new());
                                }
                                let difference_data_entity_updates =
                                    difference_data.get_mut(&new_node_path.to_string()).unwrap();
                                difference_data_entity_updates.insert(
                                    new_entity_update_type.clone(),
                                    new_entity_update_data.clone(),
                                );
                            }
                        }
                        None => {
                            if !difference_data.contains_key(&new_node_path.to_string()) {
                                difference_data.insert(new_node_path.to_string(), HashMap::new());
                            }
                            let difference_data_entity_updates =
                                difference_data.get_mut(&new_node_path.to_string()).unwrap();
                            difference_data_entity_updates.insert(
                                new_entity_update_type.clone(),
                                new_entity_update_data.clone(),
                            );
                        }
                    }
                }
            }
            None => {
                difference_data.insert(new_node_path.to_string(), new_data_entity_updates.clone());
            }
        }
    }

    difference_data
}

/// World mode component.
#[derive(Component)]

pub struct WorldMode {
    pub mode: WorldModes,
}

/// All world modes.
#[derive(Debug)]

pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}
/// For entities that are also registered with the gridmap.

pub struct GridItemData {
    pub transform_offset: Transform,
    /// So this entity can be built on a cell when another item is already present on that cell.
    pub can_be_built_with_grid_item: Vec<String>,
}

pub trait GridEntity {
    fn get_grid_item_data() -> GridItemData;
}
