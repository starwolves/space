use bevy::{prelude::{Query, Res, EventWriter, Entity, ResMut}, math::Vec3};

use crate::space_core::ecs::{atmospherics::{resources::{AtmosphericsResource, CELCIUS_KELVIN_OFFSET, MapHolders, MapHolderData}, functions::{get_atmos_id, get_atmos_index}}, pawn::components::ConnectedPlayer, map::{components::Map, events::{NetDisplayAtmospherics}, functions::{OverlayTile, get_overlay_tile_item, get_overlay_tile_priority}}, networking::resources::ReliableServerMessage, gridmap::{resources::{FOV_MAP_WIDTH, Vec2Int}, functions::gridmap_functions::world_to_cell_id}};






const MAX_VALIDS_PER_BATCH : u16 = 2500;

enum SelectedDisplayMode {
    Temperature,
    Pressure,
    Liveable,
}

pub fn atmospherics_map(
    map_holders : Query<(Entity, &Map, &ConnectedPlayer)>,
    atmospherics : Res<AtmosphericsResource>,
    mut net : EventWriter<NetDisplayAtmospherics>,
    mut display_atmos_state : ResMut<MapHolders>,
) {

    for (map_holder_entity, map_component, connected_player_component) in map_holders.iter() {

        let show_temperature;

        match &map_component.display_mode {
            Some(selected_display_mode) => {
                if selected_display_mode == "atmospherics_temperature" {
                    show_temperature=SelectedDisplayMode::Temperature;
                } else if selected_display_mode == "atmospherics_pressure" {
                    show_temperature=SelectedDisplayMode::Pressure;
                } else if selected_display_mode == "atmospherics_liveable" {
                    show_temperature=SelectedDisplayMode::Liveable;
                } else {
                    continue;
                }
            },
            None => {
                continue;
            },
        }

        let camera_center_cell_3 = world_to_cell_id(Vec3::new(map_component.camera_position.x,0.,map_component.camera_position.y));
        let camera_center_cell = Vec2Int{x:camera_center_cell_3.x,y:camera_center_cell_3.z};

        let total_cells_in_view = (map_component.view_range*2)*(map_component.view_range*2);

        

        let mut start_cam_x = camera_center_cell.x-map_component.view_range as i16;
        let mut start_cam_y = camera_center_cell.y-map_component.view_range as i16;

        if start_cam_x < -(FOV_MAP_WIDTH as i16)/2 {
            start_cam_x = -(FOV_MAP_WIDTH as i16)/2;
        }
        if start_cam_y < -(FOV_MAP_WIDTH as i16)/2 {
            start_cam_y = -(FOV_MAP_WIDTH as i16)/2;
        }

        let mut end_cam_x = camera_center_cell.x+map_component.view_range as i16;
        let mut end_cam_y = camera_center_cell.y+map_component.view_range as i16;

        if end_cam_x > FOV_MAP_WIDTH as i16/2 {
            end_cam_x = FOV_MAP_WIDTH as i16/2;
        }
        if end_cam_y > FOV_MAP_WIDTH as i16/2 {
            end_cam_y = FOV_MAP_WIDTH as i16/2;
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
                map_holder_data=d;
            },
            None => {
                display_atmos_state.holders.insert(map_holder_entity, MapHolderData{
                    batch_i: min_i,
                });
                map_holder_data=display_atmos_state.holders.get_mut(&map_holder_entity).unwrap();
            },
        }

        let mut adjusted_cell_i = map_holder_data.batch_i;

        let mut current_cell_id = get_atmos_id(adjusted_cell_i);

        if current_cell_id.x > end_cam_x {
            current_cell_id.x = start_cam_x;
            current_cell_id.y +=1;
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

            if valids_processed_i >= MAX_VALIDS_PER_BATCH || valids_processed_i >= total_cells_in_view as u16{
                break;
            }

            if first_time == false && cell_i == start_cell_i {
                break;
            }

            first_time=false;

            if cell_i >= FOV_MAP_WIDTH*FOV_MAP_WIDTH || cell_i > max_i {
                cell_i=min_i;
            }

            let mut current_cell_id = get_atmos_id(cell_i);

            if current_cell_id.x > end_cam_x {
                current_cell_id.x = start_cam_x;
                current_cell_id.y +=1;
            }
            if current_cell_id.y > end_cam_y {
                current_cell_id.y = start_cam_y;
                current_cell_id.x = start_cam_x;
            }

            cell_i = get_atmos_index(current_cell_id);

            let atmospherics = atmospherics.atmospherics.get(cell_i).unwrap();

            if atmospherics.blocked || atmospherics.flags.contains(&"default_vacuum".to_string()) {
                cell_i+=1;
                continue;
            }

            let item;

            match show_temperature {
                SelectedDisplayMode::Temperature => {
                    let tile_color = temperature_to_tile_color(atmospherics.temperature);
                    item = get_overlay_tile_item(&tile_color);
                },
                SelectedDisplayMode::Pressure => {
                    let pressure_kpa = atmospherics.get_pressure();
                    let tile_color = pressure_to_tile_color(pressure_kpa);
                    item = get_overlay_tile_item(&tile_color);
                    
                },
                SelectedDisplayMode::Liveable => {

                    let temperature_tile_color = temperature_to_tile_color(atmospherics.temperature);

                    let pressure_kpa = atmospherics.get_pressure();
                    let pressure_tile_color = pressure_to_tile_color(pressure_kpa);

                    if get_overlay_tile_priority(&temperature_tile_color) > get_overlay_tile_priority(&pressure_tile_color) {
                        item = get_overlay_tile_item(&temperature_tile_color);
                    } else {
                        item = get_overlay_tile_item(&pressure_tile_color);
                    }

                },
            } 
            

            batch.push((current_cell_id.x,current_cell_id.y, item));

            cell_i+=1;
            valids_processed_i+=1;

            adjusted_cell_i = cell_i;

        }

        net.send(NetDisplayAtmospherics {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::MapOverlayUpdate(batch),
        });

        map_holder_data.batch_i = adjusted_cell_i;


    }

    

}

fn temperature_to_tile_color(temperature : f32) -> OverlayTile {
    if temperature < -60. + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Red
    } else if temperature < -30. + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Orange
    } else if temperature < -15. + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Yellow
    } else if temperature < 39.3 + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Green
    } else if temperature < 46.3 + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Yellow
    } else if temperature < 56.3 + CELCIUS_KELVIN_OFFSET {
        OverlayTile::Orange
    } else {
        OverlayTile::Red
    }
}

fn pressure_to_tile_color(pressure_kpa:f32) -> OverlayTile {
    if pressure_kpa < 47.62275 {
        OverlayTile::Red
    } else if pressure_kpa < 75. {
        OverlayTile::Orange
    } else if pressure_kpa < 90. {
        OverlayTile::Yellow
    } else if pressure_kpa < 180.  {
        OverlayTile::Green
    } else if pressure_kpa < 200.  {
        OverlayTile::Yellow
    } else if pressure_kpa < 250.  {
        OverlayTile::Orange
    } else {
        OverlayTile::Red
    }
}
