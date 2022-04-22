use bevy_app::{App, Plugin};

use self::{
    events::InputTabAction,
    systems::{tab_action::tab_action, tab_data::tab_data},
};

pub mod components;
pub mod events;
pub mod functions;
pub mod systems;

pub struct TabActionsPlugin;

impl Plugin for TabActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tab_data)
            .add_system(tab_action)
            .add_event::<InputTabAction>();
    }
}
