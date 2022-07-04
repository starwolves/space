pub fn initialize_entity_data(
    entity_data: &mut ResMut<EntityDataResource>,
    entity_properties: EntityDataProperties,
) {
    entity_data
        .id_to_name
        .insert(entity_properties.id, entity_properties.name.clone());
    entity_data
        .name_to_id
        .insert(entity_properties.name.clone(), entity_properties.id);
    entity_data.data.push(entity_properties);
}

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

pub fn string_color_to_color(string_color: &str) -> (f32, f32, f32, f32) {
    let string_values: Vec<&str> = string_color.split(",").collect();

    let mut red_color = 0.;
    let mut green_color = 0.;
    let mut blue_color = 0.;
    let mut alpha_color = 0.;

    let mut i: u8 = 0;
    for string_value in string_values {
        match i {
            0 => {
                red_color = string_value.parse::<f32>().unwrap();
            }
            1 => {
                green_color = string_value.parse::<f32>().unwrap();
            }
            2 => {
                blue_color = string_value.parse::<f32>().unwrap();
            }
            3 => {
                alpha_color = string_value.parse::<f32>().unwrap();
            }
            _ => (),
        }

        i += 1;
    }

    (red_color, green_color, blue_color, alpha_color)
}

pub fn string_quat_to_quat(string_quad: &str) -> Quat {
    let new_string = string_quad.replace(&['(', ')', ' '][..], "");

    let string_values: Vec<&str> = new_string.split(",").collect();

    let mut x = 0.;
    let mut y = 0.;
    let mut z = 0.;
    let mut w = 0.;

    let mut i: u8 = 0;

    for string_value in string_values {
        match i {
            0 => {
                x = string_value.parse::<f32>().unwrap();
            }
            1 => {
                y = string_value.parse::<f32>().unwrap();
            }
            2 => {
                z = string_value.parse::<f32>().unwrap();
            }
            3 => {
                w = string_value.parse::<f32>().unwrap();
            }
            _ => (),
        }

        i += 1;
    }

    Quat::from_xyzw(x, y, z, w)
}

const STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_vec3_to_vec3() Error cannot parse cell id string as Vector 3.";

pub fn string_vec3_to_vec3(string_vector: &str) -> Vec3 {
    let clean_string = string_vector.replace(" ", "");

    let mut split_result: Vec<&str> = clean_string.split("(").collect();

    let mut new_string: &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    Vec3::new(
        split_result[0]
            .parse::<f32>()
            .expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[1]
            .parse::<f32>()
            .expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[2]
            .parse::<f32>()
            .expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
    )
}

const _STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_vec2_to_vec2() Error cannot parse cell id string as Vector 3.";

pub fn _string_vec2_to_vec2_int(string_vector: &str) -> Vec2Int {
    let clean_string = string_vector.replace(" ", "");

    let mut split_result: Vec<&str> = clean_string.split("(").collect();

    let mut new_string: &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    Vec2Int {
        x: split_result[0]
            .parse::<f32>()
            .expect(_STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE) as i16,
        y: split_result[1]
            .parse::<f32>()
            .expect(_STRING_VEC2_TO_VEC2_CANNOT_PARSE_MESSAGE) as i16,
    }
}

const STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE: &str =
    "main.rs string_transform_to_transform() Error cannot parse floats of transform.";

pub fn string_transform_to_transform(string_transform: &str) -> Transform {
    let mut split_result: Vec<&str> = string_transform.split(",").collect();

    let mut odd_index_value: usize = 0;

    for seperated_value in split_result.iter() {
        if seperated_value.contains(" - ") {
            let odd_values: Vec<&str> = seperated_value.split(" - ").collect();

            split_result.remove(odd_index_value);

            split_result.insert(8, odd_values[0]);
            split_result.insert(9, odd_values[1]);

            break;
        }

        odd_index_value += 1;
    }

    let mut current_index: usize = 0;

    let mut clean_strings: Vec<String> = Vec::new();

    for seperated_value in split_result.iter() {
        clean_strings.insert(current_index, seperated_value.replace(' ', ""));

        current_index += 1;
    }

    let translation = Vec3::new(
        clean_strings[9]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[10]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[11]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let basis_x = Vec3::new(
        clean_strings[0]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[3]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[6]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let basis_y = Vec3::new(
        clean_strings[1]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[4]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[7]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let basis_z = Vec3::new(
        clean_strings[2]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[5]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
        clean_strings[8]
            .parse::<f32>()
            .expect(STRING_TRANSFORM_TO_TRANSFORM_CANNOT_PARSE_MESSAGE),
    );

    let mut transform_result = Transform::from_translation(translation);
    transform_result.rotation = Quat::from_mat3(&Mat3::from_cols(basis_x, basis_y, basis_z));
    // Hard coding scale, not required for current use case
    transform_result.scale = Vec3::new(1., 1., 1.);

    transform_result
}

use bevy::{
    core::{FixedTimesteps, Time},
    math::{Mat3, Quat, Vec3},
    prelude::{warn, Component, Entity, Query, Res, ResMut, Transform},
};
use bevy_rapier3d::{
    na::Quaternion,
    rapier::math::{Isometry, Real, Rotation, Translation},
};

pub fn transform_to_isometry(transform: Transform) -> Isometry<Real> {
    let translation: Translation<f32> = Vec3::new(
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
    )
    .into();

    let quaternion = Quaternion::new(
        transform.rotation.w,
        transform.rotation.x,
        transform.rotation.y,
        transform.rotation.z,
    );

    let rotation = Rotation::from_quaternion(quaternion);

    Isometry::<Real> {
        translation: translation,
        rotation: rotation,
    }
}

pub fn isometry_to_transform(isometry: Isometry<Real>) -> Transform {
    let translation = Vec3::new(
        isometry.translation.x,
        isometry.translation.y,
        isometry.translation.z,
    );

    let rotation = Quat::from_xyzw(
        isometry.rotation.i,
        isometry.rotation.j,
        isometry.rotation.k,
        isometry.rotation.w,
    );

    Transform {
        translation: translation,
        rotation: rotation,
        scale: Vec3::new(1., 1., 1.),
    }
}

use bevy_renet::renet::RenetServer;
use bincode::serialize;

use crate::core::{
    connected_player::plugin::HandleToEntity,
    gridmap::gridmap::Vec2Int,
    networking::{
        networking::{EntityUpdateData, ReliableServerMessage, UnreliableServerMessage},
        plugin::RENET_UNRELIABLE_CHANNEL_ID,
    },
    rigid_body::{
        broadcast_interpolation_transforms::UpdateTransform, rigid_body::CachedBroadcastTransform,
    },
    sensable::sensable::Sensable,
};

pub const INTERPOLATION_LABEL1: &str = "fixed_timestep_interpolation1";

pub fn broadcast_position_updates(
    time: Res<Time>,
    fixed_timesteps: Res<FixedTimesteps>,

    mut net: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut query_update_transform_entities: Query<(
        Entity,
        &Sensable,
        &UpdateTransform,
        &Transform,
        &mut CachedBroadcastTransform,
    )>,
) {
    let current_time_stamp = time.time_since_startup().as_millis();

    let overstep_percentage = fixed_timesteps
        .get(INTERPOLATION_LABEL1)
        .unwrap()
        .overstep_percentage();
    if overstep_percentage > 5. {
        if current_time_stamp > 60000 {
            warn!("overstep_percentage: {}", overstep_percentage);
        }
    }

    for (
        entity,
        visible_component,
        _update_transform_component,
        static_transform_component,
        mut cached_transform_component,
    ) in query_update_transform_entities.iter_mut()
    {
        if cached_transform_component.transform == *static_transform_component {
            continue;
        }

        cached_transform_component.transform = *static_transform_component;

        let new_position = static_transform_component.translation;

        for sensed_by_entity in visible_component.sensed_by.iter() {
            let player_handle_option = handle_to_entity.inv_map.get(&sensed_by_entity);

            match player_handle_option {
                Some(handle) => {
                    net.send_message(
                        *handle,
                        RENET_UNRELIABLE_CHANNEL_ID,
                        serialize::<UnreliableServerMessage>(
                            &UnreliableServerMessage::PositionUpdate(
                                entity.to_bits(),
                                new_position,
                                current_time_stamp as u64,
                            ),
                        )
                        .unwrap(),
                    );
                }
                None => {
                    continue;
                }
            }
        }
    }
}

use std::collections::HashMap;

use crate::core::{
    connected_player::connection::ConnectedPlayer, pawn::pawn::PersistentPlayerData,
};

use super::spawn::RawEntity;

#[derive(Default)]
pub struct EntityDataResource {
    pub data: Vec<EntityDataProperties>,
    pub incremented_id: usize,
    pub id_to_name: HashMap<usize, String>,
    pub name_to_id: HashMap<String, usize>,
}

impl EntityDataResource {
    pub fn get_id_inc(&mut self) -> usize {
        let return_val = self.incremented_id.clone();
        self.incremented_id += 1;
        return_val
    }
}

#[derive(Clone)]
pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}

#[derive(Clone)]
pub struct SpawnPawnData {
    pub persistent_player_data: PersistentPlayerData,
    pub connected_player_option: Option<ConnectedPlayer>,
    pub inventory_setup: Vec<(String, String)>,
    pub designation: PawnDesignation,
}

pub struct EntityDataProperties {
    pub name: String,
    pub id: usize,
    pub grid_item: Option<GridItemData>,
}

pub struct GridItemData {
    pub transform_offset: Transform,
    pub can_be_built_with_grid_item: Vec<String>,
}

impl Default for EntityDataProperties {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            grid_item: None,
        }
    }
}

#[derive(Clone)]
pub struct ShowcaseData {
    pub handle: u64,
}

pub struct RawSpawnEvent {
    pub raw_entity: RawEntity,
}

pub struct NetShowcase {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

#[derive(Component)]
pub struct Server;

#[derive(Component)]
pub struct Showcase {
    pub handle: u64,
}

#[derive(Component)]
pub struct DefaultMapEntity;

#[derive(Component)]
pub struct EntityData {
    pub entity_class: String,
    pub entity_name: String,
    pub entity_group: EntityGroup,
}

#[derive(Copy, Clone)]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn,
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_class: "".to_string(),
            entity_name: "".to_string(),
            entity_group: EntityGroup::None,
        }
    }
}
