use bevy::prelude::{App, ClearColor, IntoSystemDescriptor, Plugin, SystemLabel};
use resources::is_server::is_server;

use crate::{
    build::{
        auto_fill_connect_menu, on_submenu_connect_creation, show_main_menu, show_play_menu,
        startup_show_menu, AutoFillConnectSubMenu, EnableMainMenu, EnablePlayMenu, MainMenuLabel,
        MainMenuState, PlayMenuState, MAIN_BG_COLOR,
    },
    events::{button_presses, connect_to_server_button, space_frontiers_link, starwolves_link},
    hide::{confirm_connection, hide_main_menu},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "client")]
pub enum StartupLabel {
    Live,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        if is_server() == false {
            app.add_system(show_main_menu.label(MainMenuLabel::BuildMainMenu))
                .add_system(hide_main_menu)
                .add_event::<EnableMainMenu>()
                .init_resource::<MainMenuState>()
                .add_startup_system(startup_show_menu)
                .insert_resource(ClearColor(MAIN_BG_COLOR))
                .add_system(button_presses)
                .add_event::<EnablePlayMenu>()
                .add_system(show_play_menu.before(MainMenuLabel::BuildMainMenu))
                .init_resource::<PlayMenuState>()
                .add_system(starwolves_link)
                .add_system(space_frontiers_link)
                .add_system(connect_to_server_button)
                .add_system(auto_fill_connect_menu)
                .add_event::<AutoFillConnectSubMenu>()
                .add_system(on_submenu_connect_creation)
                .add_system(confirm_connection);
        }
    }
}
