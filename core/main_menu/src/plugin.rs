use bevy::prelude::{App, ClearColor, IntoSystemConfigs, Plugin, Startup, SystemSet, Update};
use hud::mouse::{grab_cursor, release_cursor};
use resources::is_server::is_server;

use crate::{
    build::{
        auto_fill_connect_menu, on_submenu_connect_creation, show_main_menu, show_play_menu,
        startup_show_menu, AutoFillConnectSubMenu, EnableMainMenu, EnablePlayMenu, MainMenuLabel,
        MainMenuState, PlayMenuState, MAIN_BG_COLOR,
    },
    events::{
        button_presses, connect_to_server_button, space_frontiers_link, starwolves_link,
        toggle_esc_menu,
    },
    hide::{confirm_connection, hide_main_menu},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum StartupLabel {
    Live,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        if is_server() == false {
            app.add_systems(
                Update,
                (
                    show_main_menu.in_set(MainMenuLabel::BuildMainMenu),
                    hide_main_menu,
                    button_presses,
                    starwolves_link,
                    space_frontiers_link,
                    connect_to_server_button,
                    auto_fill_connect_menu,
                    on_submenu_connect_creation,
                    confirm_connection,
                    toggle_esc_menu.after(grab_cursor).after(release_cursor),
                    show_play_menu.before(MainMenuLabel::BuildMainMenu),
                ),
            )
            .add_event::<EnableMainMenu>()
            .init_resource::<MainMenuState>()
            .add_systems(Startup, startup_show_menu)
            .insert_resource(ClearColor(MAIN_BG_COLOR))
            .add_event::<EnablePlayMenu>()
            .init_resource::<PlayMenuState>()
            .add_event::<AutoFillConnectSubMenu>();
        }
    }
}
