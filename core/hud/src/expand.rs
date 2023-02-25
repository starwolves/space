use bevy::{
    prelude::{warn, EventReader, Query, ResMut},
    ui::{Display, Size, Style, Val},
};
use cameras::controllers::fps::CameraMouseInputEnabled;
use resources::hud::HudState;

use crate::hud::{
    ExpandedLeftContentHud, LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH, LEFT_RIGHT_EDGE_HUD_WIDTH,
};

/// Event to expand the hud.
pub struct ExpandInventoryHud {
    pub expand: bool,
}

pub(crate) fn expand_hud(
    mut events: EventReader<ExpandInventoryHud>,
    mut state: ResMut<HudState>,
    mut mouse_enabled: ResMut<CameraMouseInputEnabled>,
    mut node_query: Query<&mut Style>,
    mut expand_event: EventReader<ExpandedLeftContentHud>,
) {
    for event in expand_event.iter() {
        if event.expanded {
            match node_query.get_mut(state.left_content_node) {
                Ok(mut style) => {
                    style.display = Display::Flex;
                }
                Err(_) => {
                    warn!("Couldnt find left content node.");
                }
            }
            match node_query.get_mut(state.right_content_node) {
                Ok(mut style) => {
                    style.display = Display::Flex;
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match node_query.get_mut(state.left_edge_node) {
                Ok(mut style) => {
                    style.size =
                        Size::new(Val::Percent(LEFT_RIGHT_EDGE_HUD_WIDTH), Val::Percent(100.));
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match node_query.get_mut(state.right_edge_node) {
                Ok(mut style) => {
                    style.size =
                        Size::new(Val::Percent(LEFT_RIGHT_EDGE_HUD_WIDTH), Val::Percent(100.));
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match node_query.get_mut(state.center_content_node) {
                Ok(mut style) => {
                    style.size = Size::new(Val::Percent(50.), Val::Percent(100.));
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
        } else {
            match node_query.get_mut(state.left_content_node) {
                Ok(mut style) => {
                    style.display = Display::None;
                }
                Err(_) => {
                    warn!("Couldnt find left content node.");
                }
            }
            match node_query.get_mut(state.right_content_node) {
                Ok(mut style) => {
                    style.display = Display::None;
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match node_query.get_mut(state.left_edge_node) {
                Ok(mut style) => {
                    style.size = Size::new(
                        Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH),
                        Val::Percent(100.),
                    );
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
            match node_query.get_mut(state.right_edge_node) {
                Ok(mut style) => {
                    style.size = Size::new(
                        Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH),
                        Val::Percent(100.),
                    );
                }
                Err(_) => {
                    warn!("Couldnt find right content node.");
                }
            }
        }
    }

    for event in events.iter() {
        state.expanded = event.expand;
        mouse_enabled.enabled = !event.expand;
    }
}
