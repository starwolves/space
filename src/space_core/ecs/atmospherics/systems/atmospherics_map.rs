use bevy::prelude::{Query, Res, EventWriter, Local};

use crate::space_core::ecs::{atmospherics::{resources::{AtmosphericsResource, CELCIUS_KELVIN_OFFSET}, functions::get_atmos_id}, pawn::components::ConnectedPlayer, map::{components::Map, events::{NetDisplayAtmospherics}, functions::{OverlayTile, get_overlay_tile_item, get_overlay_tile_priority}}, networking::resources::ReliableServerMessage, gridmap::resources::FOV_MAP_WIDTH};




#[derive(Default)]
pub struct MapHolderData {
    pub current_cell_i : usize
}

const MAX_VALIDS_PER_BATCH : u16 = 2500;

enum SelectedDisplayMode {
    Temperature,
    Pressure,
    Liveable,
}

pub fn atmospherics_map(
    map_holders : Query<(&Map, &ConnectedPlayer)>,
    atmospherics : Res<AtmosphericsResource>,
    mut net : EventWriter<NetDisplayAtmospherics>,
    mut display_atmos_state : Local<MapHolderData>,
) {

    let mut adjusted_cell_i = display_atmos_state.current_cell_i;

    for (map_component, connected_player_component) in map_holders.iter() {

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

        let mut batch = vec![];

        let mut valids_processed_i = 0;
        let mut cell_i = display_atmos_state.current_cell_i;
        loop {

            if valids_processed_i >= MAX_VALIDS_PER_BATCH {
                break;
            }

            if cell_i >= FOV_MAP_WIDTH*FOV_MAP_WIDTH {
                cell_i=0;
            }

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
            

            let id = get_atmos_id(cell_i);

            batch.push((id.x,id.y, item));

            cell_i+=1;
            valids_processed_i+=1;

            adjusted_cell_i = cell_i;

        }

        net.send(NetDisplayAtmospherics {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::MapOverlayUpdate(batch),
        });


    }

    display_atmos_state.current_cell_i = adjusted_cell_i;

    if display_atmos_state.current_cell_i >= FOV_MAP_WIDTH*FOV_MAP_WIDTH {
        display_atmos_state.current_cell_i= display_atmos_state.current_cell_i-FOV_MAP_WIDTH*FOV_MAP_WIDTH;
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
