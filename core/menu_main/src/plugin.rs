use bevy::prelude::{App, Plugin, SystemLabel};

use crate::{
    build::{show_main_menu, startup_show_menu, EnableMainMenu, MainMenuState},
    hide::hide_main_menu,
    ui_events::{button_presses, hover_visuals},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "client")]
pub enum StartupLabel {
    Live,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "client") {
            app.add_system(show_main_menu)
                .add_system(hide_main_menu)
                .add_event::<EnableMainMenu>()
                .init_resource::<MainMenuState>()
                .add_startup_system(startup_show_menu)
                .add_system(hover_visuals)
                .add_system(button_presses);
        }
    }
}
