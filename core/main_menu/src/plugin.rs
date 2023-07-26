use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, SystemSet};
use hud::mouse::{grab_cursor, release_cursor};
use networking::client::token_assign_server;
use resources::{is_server::is_server, sets::MainSet};

use crate::{
    build::{
        auto_fill_connect_menu, on_submenu_connect_creation, show_main_menu, show_play_menu,
        startup_show_menu, AutoFillConnectSubMenu, EnableMainMenu, EnablePlayMenu, MainMenuLabel,
        MainMenuState, PlayMenuState,
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
                FixedUpdate,
                (
                    show_main_menu.in_set(MainMenuLabel::BuildMainMenu),
                    hide_main_menu,
                    button_presses,
                    starwolves_link,
                    space_frontiers_link,
                    connect_to_server_button.before(token_assign_server),
                    auto_fill_connect_menu,
                    on_submenu_connect_creation,
                    confirm_connection.before(hide_main_menu),
                    toggle_esc_menu.after(grab_cursor).after(release_cursor),
                    show_play_menu.before(MainMenuLabel::BuildMainMenu),
                )
                    .in_set(MainSet::Update),
            )
            .add_event::<EnableMainMenu>()
            .init_resource::<MainMenuState>()
            .add_systems(
                FixedUpdate,
                startup_show_menu
                    .in_set(MainSet::Update)
                    .before(hide_main_menu),
            )
            .add_event::<EnablePlayMenu>()
            .init_resource::<PlayMenuState>()
            .add_event::<AutoFillConnectSubMenu>();
        }
    }
}
