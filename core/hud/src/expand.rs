use bevy::{
    prelude::{warn, Entity, Event, EventReader, Query, ResMut, With},
    ui::{Display, Style, Val},
};
use resources::hud::HudState;

use crate::{
    communication::{
        build::{MESSAGES_DEFAULT_MAX_WIDTH, MESSAGES_DEFAULT_MIN_WIDTH},
        console::CommunicationTextBundle,
    },
    hud::{ExpandedLeftContentHud, LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH, LEFT_RIGHT_EDGE_HUD_WIDTH},
};

/// Event to expand the hud.
#[derive(Event)]
pub struct ExpandInventoryHud {
    pub expand: bool,
}

pub(crate) fn expand_inventory_hud(
    mut events: EventReader<ExpandInventoryHud>,
    mut state: ResMut<HudState>,
    mut style_query: Query<&mut Style>,
    mut expand_event: EventReader<ExpandedLeftContentHud>,
    text_query: Query<Entity, With<CommunicationTextBundle>>,
) {
    for event in expand_event.iter() {
        if event.expanded {
            for ent in text_query.iter() {
                match style_query.get_mut(ent) {
                    Ok(mut st) => {
                        st.max_width = Val::Px(MESSAGES_DEFAULT_MIN_WIDTH);
                        st.max_height = Val::Px(0.);
                    }
                    Err(_) => {
                        warn!("Couldnt find style.");
                    }
                }
            }

            match style_query.get_mut(state.left_content_node) {
                Ok(mut style) => {
                    style.display = Display::Flex;
                }
                Err(_) => {
                    warn!("Couldnt find left content node.");
                }
            }
            match style_query.get_mut(state.right_content_node) {
                Ok(mut style) => {
                    style.display = Display::Flex;
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match style_query.get_mut(state.left_edge_node) {
                Ok(mut style) => {
                    style.width = Val::Percent(LEFT_RIGHT_EDGE_HUD_WIDTH);
                    style.height = Val::Percent(100.);
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match style_query.get_mut(state.right_edge_node) {
                Ok(mut style) => {
                    style.width = Val::Percent(LEFT_RIGHT_EDGE_HUD_WIDTH);
                    style.height = Val::Percent(100.);
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match style_query.get_mut(state.center_content_node) {
                Ok(mut style) => {
                    style.width = Val::Percent(50.);
                    style.height = Val::Percent(100.);
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
        } else {
            for ent in text_query.iter() {
                match style_query.get_mut(ent) {
                    Ok(mut st) => {
                        st.max_width = Val::Px(MESSAGES_DEFAULT_MAX_WIDTH);
                        st.max_height = Val::Px(0.);
                    }
                    Err(_) => {
                        warn!("Couldnt find style.");
                    }
                }
            }
            match style_query.get_mut(state.left_content_node) {
                Ok(mut style) => {
                    style.display = Display::None;
                }
                Err(_) => {
                    warn!("Couldnt find left content node.");
                }
            }
            match style_query.get_mut(state.right_content_node) {
                Ok(mut style) => {
                    style.display = Display::None;
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match style_query.get_mut(state.left_edge_node) {
                Ok(mut style) => {
                    style.width = Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH);
                    style.height = Val::Percent(100.);
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match style_query.get_mut(state.right_edge_node) {
                Ok(mut style) => {
                    style.width = Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH);
                    style.height = Val::Percent(100.);
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
        }
    }

    for event in events.iter() {
        state.expanded = event.expand;
    }
}
