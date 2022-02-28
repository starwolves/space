use std::collections::HashSet;

use bevy_internal::{prelude::{Query, Entity, Res, EventWriter, ResMut}, math::Vec3};

use crate::space::core::{
    atmospherics::{
        events::NetMapDisplayAtmospherics,
        functions::{get_atmos_id, get_atmos_index},
        resources::{AtmosphericsResource, MapHolderData, MapHolders, CELCIUS_KELVIN_OFFSET},
    },
    gridmap::{
        functions::gridmap_functions::world_to_cell_id,
        resources::{Vec2Int, FOV_MAP_WIDTH},
    },
    map::{
        components::Map,
        functions::{get_overlay_tile_item, get_overlay_tile_priority, OverlayTile},
    },
    networking::resources::{NetMessageType, ReliableServerMessage},
    pawn::components::ConnectedPlayer,
};

const MAX_VALIDS_PER_BATCH: u16 = 2500;

enum SelectedDisplayMode {
    Temperature,
    Pressure,
    Liveable,
}

pub fn atmospherics_map(
    map_holders: Query<(Entity, &Map, &ConnectedPlayer)>,
    atmospherics: Res<AtmosphericsResource>,
    mut net: EventWriter<NetMapDisplayAtmospherics>,
    mut display_atmos_state: ResMut<MapHolders>,
) {
    for (map_holder_entity, map_component, connected_player_component) in map_holders.iter() {
        let show_temperature;

        match &map_component.display_mode {
            Some(selected_display_mode) => {
                if selected_display_mode == "atmospherics_temperature" {
                    show_temperature = SelectedDisplayMode::Temperature;
                } else if selected_display_mode == "atmospherics_pressure" {
                    show_temperature = SelectedDisplayMode::Pressure;
                } else if selected_display_mode == "atmospherics_liveable" {
                    show_temperature = SelectedDisplayMode::Liveable;
                } else {
                    continue;
                }
            }
            None => {
                continue;
            }
        }

        let camera_center_cell_3 = world_to_cell_id(Vec3::new(
            map_component.camera_position.x,
            0.,
            map_component.camera_position.y,
        ));
        let camera_center_cell = Vec2Int {
            x: camera_center_cell_3.x,
            y: camera_center_cell_3.z,
        };

        let mut start_cam_x = camera_center_cell.x - map_component.view_range as i16;
        let mut start_cam_y = camera_center_cell.y - map_component.view_range as i16;

        if start_cam_x < -(FOV_MAP_WIDTH as i16) / 2 {
            start_cam_x = -(FOV_MAP_WIDTH as i16) / 2;
        }
        if start_cam_y < -(FOV_MAP_WIDTH as i16) / 2 {
            start_cam_y = -(FOV_MAP_WIDTH as i16) / 2;
        }

        let mut end_cam_x = camera_center_cell.x + map_component.view_range as i16;
        let mut end_cam_y = camera_center_cell.y + map_component.view_range as i16;

        if end_cam_x > FOV_MAP_WIDTH as i16 / 2 {
            end_cam_x = FOV_MAP_WIDTH as i16 / 2;
        }
        if end_cam_y > FOV_MAP_WIDTH as i16 / 2 {
            end_cam_y = FOV_MAP_WIDTH as i16 / 2;
        }

        let min_i = get_atmos_index(Vec2Int {
            x: start_cam_x,
            y: start_cam_y,
        });
        let max_i = get_atmos_index(Vec2Int {
            x: end_cam_x,
            y: end_cam_y,
        });

        let mut map_holder_data;

        match display_atmos_state.holders.get_mut(&map_holder_entity) {
            Some(d) => {
                map_holder_data = d;
            }
            None => {
                display_atmos_state.holders.insert(
                    map_holder_entity,
                    MapHolderData {
                        batch_i: min_i,
                        ..Default::default()
                    },
                );
                map_holder_data = display_atmos_state
                    .holders
                    .get_mut(&map_holder_entity)
                    .unwrap();
            }
        }

        let total_cells_in_view = (map_component.view_range * 2) * (map_component.view_range * 2);

        let mut adjusted_cell_i = map_holder_data.batch_i;

        let mut current_cell_id = get_atmos_id(adjusted_cell_i);

        if current_cell_id.x > end_cam_x {
            current_cell_id.x = start_cam_x;
            current_cell_id.y += 1;
        }
        if current_cell_id.y > end_cam_y {
            current_cell_id.y = start_cam_y;
            current_cell_id.x = start_cam_x;
        }

        adjusted_cell_i = get_atmos_index(current_cell_id);

        let mut first_time = true;

        let mut batch = vec![];

        let mut valids_processed_i = 0;
        let mut cell_i = adjusted_cell_i;
        let start_cell_i = cell_i;

        loop {
            if valids_processed_i >= MAX_VALIDS_PER_BATCH
                || valids_processed_i >= total_cells_in_view as u16
            {
                break;
            }

            if first_time == false && cell_i == start_cell_i {
                break;
            }

            first_time = false;

            if cell_i >= FOV_MAP_WIDTH * FOV_MAP_WIDTH || cell_i > max_i {
                cell_i = min_i;
            }

            let mut current_cell_id = get_atmos_id(cell_i);

            if current_cell_id.x > end_cam_x {
                current_cell_id.x = start_cam_x;
                current_cell_id.y += 1;
            }
            if current_cell_id.y > end_cam_y {
                current_cell_id.y = start_cam_y;
                current_cell_id.x = start_cam_x;
            }

            cell_i = get_atmos_index(current_cell_id);

            let atmospherics = atmospherics.atmospherics.get(cell_i).unwrap();

            let atmospherics_cache = map_holder_data.cache.get_mut(cell_i).unwrap();

            if atmospherics.flags.contains(&"default_vacuum".to_string()) {
                cell_i += 1;
                if atmospherics_cache.tile_color.is_some() {
                    atmospherics_cache.tile_color = None;
                    batch.push((current_cell_id.x, current_cell_id.y, -1));
                }
                continue;
            }

            if atmospherics.blocked {
                cell_i += 1;
                continue;
            }

            let item;
            let new_tile_color;

            match show_temperature {
                SelectedDisplayMode::Temperature => {
                    let tile_color = temperature_to_tile_color(atmospherics.temperature);
                    item = get_overlay_tile_item(&tile_color);
                    new_tile_color = tile_color;
                }
                SelectedDisplayMode::Pressure => {
                    let pressure_kpa = atmospherics.get_pressure();
                    let tile_color = pressure_to_tile_color(pressure_kpa);
                    item = get_overlay_tile_item(&tile_color);
                    new_tile_color = tile_color;
                }
                SelectedDisplayMode::Liveable => {
                    let temperature_tile_color =
                        temperature_to_tile_color(atmospherics.temperature);

                    let pressure_kpa = atmospherics.get_pressure();
                    let pressure_tile_color = pressure_to_tile_color(pressure_kpa);

                    if get_overlay_tile_priority(&temperature_tile_color)
                        > get_overlay_tile_priority(&pressure_tile_color)
                    {
                        item = get_overlay_tile_item(&temperature_tile_color);
                        new_tile_color = temperature_tile_color;
                    } else {
                        item = get_overlay_tile_item(&pressure_tile_color);
                        new_tile_color = pressure_tile_color;
                    }
                }
            }

            let should_update;

            match &atmospherics_cache.tile_color {
                Some(r) => {
                    if r.clone() != new_tile_color {
                        should_update = true;
                    } else {
                        should_update = false;
                    }
                }
                None => {
                    should_update = true;
                }
            }

            if should_update {
                batch.push((current_cell_id.x, current_cell_id.y, item));
                atmospherics_cache.tile_color = Some(new_tile_color);
            }

            cell_i += 1;
            valids_processed_i += 1;

            adjusted_cell_i = cell_i;
        }

        net.send(NetMapDisplayAtmospherics {
            handle: connected_player_component.handle,
            message: NetMessageType::Reliable(ReliableServerMessage::MapOverlayUpdate(batch)),
        });

        map_holder_data.batch_i = adjusted_cell_i;

        // Vector that gets the difference in FOV cells, the substractives. And that has them in a vector of indexes
        // So we efficiently remove them here
        // We store prev cam pos and prev cam distance, get all its i's in a vector. Do the same with current cam and get difference.

        let mut prev_start_cam_x =
            map_holder_data.prev_camera_cell_id.x - map_holder_data.prev_camera_view_range as i16;
        let mut prev_start_cam_y =
            map_holder_data.prev_camera_cell_id.y - map_holder_data.prev_camera_view_range as i16;

        if prev_start_cam_x < -(FOV_MAP_WIDTH as i16) / 2 {
            prev_start_cam_x = -(FOV_MAP_WIDTH as i16) / 2;
        }
        if prev_start_cam_y < -(FOV_MAP_WIDTH as i16) / 2 {
            prev_start_cam_y = -(FOV_MAP_WIDTH as i16) / 2;
        }

        let mut prev_end_cam_x =
            map_holder_data.prev_camera_cell_id.x + map_holder_data.prev_camera_view_range as i16;
        let mut prev_end_cam_y =
            map_holder_data.prev_camera_cell_id.y + map_holder_data.prev_camera_view_range as i16;

        if prev_end_cam_x > FOV_MAP_WIDTH as i16 / 2 {
            prev_end_cam_x = FOV_MAP_WIDTH as i16 / 2;
        }
        if prev_end_cam_y > FOV_MAP_WIDTH as i16 / 2 {
            prev_end_cam_y = FOV_MAP_WIDTH as i16 / 2;
        }

        let mut prev_cell_is = vec![];

        let mut prev_iter_id = Vec2Int {
            x: prev_start_cam_x - 1,
            y: prev_start_cam_y,
        };

        for _i in 0..(map_holder_data.prev_camera_view_range * 2)
            * (map_holder_data.prev_camera_view_range * 2)
        {
            prev_iter_id.x += 1;
            if prev_iter_id.x > prev_end_cam_x {
                prev_iter_id.x = prev_start_cam_x;
                prev_iter_id.y += 1;
            }
            if prev_iter_id.y > prev_end_cam_y {
                break;
            }

            prev_cell_is.push(get_atmos_index(prev_iter_id));
        }

        let mut new_cell_is = vec![];

        let mut new_iter_id = Vec2Int {
            x: start_cam_x - 1,
            y: start_cam_y,
        };

        for _i in 0..(map_component.view_range * 2) * (map_component.view_range * 2) {
            new_iter_id.x += 1;
            if new_iter_id.x > end_cam_x {
                new_iter_id.x = start_cam_x;
                new_iter_id.y += 1;
            }
            if new_iter_id.y > end_cam_y {
                break;
            }

            new_cell_is.push(get_atmos_index(new_iter_id));
        }

        let item_set: HashSet<_> = prev_cell_is.iter().collect();

        let difference: Vec<_>;

        if map_holder_data.reset_cache {
            map_holder_data.reset_cache = false;
            difference = new_cell_is;
        } else {
            difference = new_cell_is
                .into_iter()
                .filter(|item| !item_set.contains(item))
                .collect();
        }

        for i in difference {
            // If outside of FOV put tile color to none as client resets it too.
            if i >= FOV_MAP_WIDTH * FOV_MAP_WIDTH {
                continue;
            }
            let atmos_data = map_holder_data.cache.get_mut(i).unwrap();
            atmos_data.tile_color = None;
        }

        map_holder_data.prev_camera_cell_id = camera_center_cell;
        map_holder_data.prev_camera_view_range = map_component.view_range;
    }
}

pub const MINIMUM_LIVABLE_TEMPERATURE: f32 = -22. + CELCIUS_KELVIN_OFFSET;
pub const MAXIMUM_LIVABLE_TEMPERATURE: f32 = 39.3 + CELCIUS_KELVIN_OFFSET;

fn temperature_to_tile_color(temperature: f32) -> OverlayTile {
    if temperature < -40. + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Red
    } else if temperature < -33. + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Orange
    } else if temperature < MINIMUM_LIVABLE_TEMPERATURE {
        OverlayTile::Yellow
    } else if temperature < MAXIMUM_LIVABLE_TEMPERATURE {
        OverlayTile::Green
    } else if temperature < 46.3 + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Yellow
    } else if temperature < 52.3 + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Orange
    } else {
        OverlayTile::Red
    }
}

pub const MINIMUM_LIVABLE_PRESSURE: f32 = 90.;
pub const MAXIMUM_LIVABLE_PRESSURE: f32 = 180.;

fn pressure_to_tile_color(pressure_kpa: f32) -> OverlayTile {
    if pressure_kpa < 47.62275 {
        OverlayTile::Red
    } else if pressure_kpa < 75. {
        OverlayTile::Orange
    } else if pressure_kpa < MINIMUM_LIVABLE_PRESSURE {
        OverlayTile::Yellow
    } else if pressure_kpa < MAXIMUM_LIVABLE_PRESSURE {
        OverlayTile::Green
    } else if pressure_kpa < 200. {
        OverlayTile::Yellow
    } else if pressure_kpa < 250. {
        OverlayTile::Orange
    } else {
        OverlayTile::Red
    }
}
