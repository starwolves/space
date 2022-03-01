use bevy_app::EventReader;
use bevy_ecs::system::{Query, ResMut};

use crate::space::core::{
    atmospherics::resources::MapHolders,
    data_link::components::{DataLink, DataLinkType},
    map::{components::Map, events::InputMapChangeDisplayMode},
};

pub fn change_display_mode(
    mut input_display_mode_changes: EventReader<InputMapChangeDisplayMode>,
    mut map_holders: Query<(&mut Map, &DataLink)>,
    mut display_atmos_state: ResMut<MapHolders>,
) {
    for event in input_display_mode_changes.iter() {
        let mut map_component;
        let data_link_component;

        match map_holders.get_mut(event.entity) {
            Ok((m, d)) => {
                map_component = m;
                data_link_component = d;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut found = false;
        for (_name, id) in map_component.available_display_modes.iter() {
            if id == &event.display_mode {
                found = true;
                break;
            }
        }

        if !found {
            continue;
        }

        if event.display_mode == "atmospherics_temperature"
            || event.display_mode == "atmospherics_pressure"
            || event.display_mode == "atmospherics_liveable"
        {
            if data_link_component
                .links
                .contains(&DataLinkType::FullAtmospherics)
            {
                map_component.display_mode = Some(event.display_mode.clone());
                match display_atmos_state.holders.get_mut(&event.entity) {
                    Some(atmospherics_map_state) => {
                        atmospherics_map_state.reset_cache = true;
                    }
                    None => {}
                }
            }
        } else {
            display_atmos_state.holders.remove(&event.entity);
        }

        if event.display_mode == "standard" {
            map_component.display_mode = None;
        }
    }
}
