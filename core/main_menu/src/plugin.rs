use bevy::{
    app::{PostStartup, Startup},
    prelude::{App, IntoSystemConfigs, Plugin, SystemSet},
};
use entity::despawn::DespawnEntitySet;
use networking::client::token_assign_server;
use resources::{
    modes::is_server_mode,
    ordering::{PreUpdate, Update},
};
use ui::{
    cursor::{grab_cursor, release_cursor},
    text_input::TextInputSet,
};

use crate::{
    build::{
        auto_fill_connect_menu, buffer_play_menu, on_submenu_connect_creation, show_main_menu,
        show_play_menu, startup_show_menu, AutoFillConnectSubMenu, EnableMainMenu, EnablePlayMenu,
        EnablePlayMenuBuffer, MainMenuLabel, PlayMenuState,
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
        if !is_server_mode(app) {
            app.add_systems(PostStartup, show_main_menu);
            app.add_systems(
                Update,
                (
                    show_main_menu
                        .in_set(MainMenuLabel::BuildMainMenu)
                        .in_set(MainMenuLabel::Play),
                    hide_main_menu.before(DespawnEntitySet),
                    button_presses.in_set(MainMenuLabel::Play),
                    starwolves_link,
                    space_frontiers_link,
                    connect_to_server_button.before(token_assign_server),
                    auto_fill_connect_menu
                        .after(on_submenu_connect_creation)
                        .before(TextInputSet::Set),
                    on_submenu_connect_creation,
                    confirm_connection
                        .before(hide_main_menu)
                        .before(MainMenuLabel::BuildMainMenu),
                    toggle_esc_menu.after(grab_cursor).after(release_cursor),
                    show_play_menu.after(MainMenuLabel::Play),
                ),
            )
            .add_event::<EnableMainMenu>()
            .add_systems(
                Startup,
                startup_show_menu
                    .before(hide_main_menu)
                    .before(MainMenuLabel::BuildMainMenu),
            )
            .add_systems(PreUpdate, buffer_play_menu)
            .add_event::<EnablePlayMenu>()
            .init_resource::<PlayMenuState>()
            .add_event::<AutoFillConnectSubMenu>()
            .init_resource::<EnablePlayMenuBuffer>();
        }
    }
}
