use crate::build::MainMenuPlayButton;
use crate::build::SIDEBAR_COLOR;
use crate::build::{MainMenuExitButton, MainMenuSettingsButton};
use bevy::prelude::warn;
use bevy::prelude::Entity;
use bevy::prelude::Parent;
use bevy::prelude::{Changed, Color};
use bevy::ui::UiColor;
use bevy::{
    prelude::{Button, Query, With},
    ui::Interaction,
};

use bevy::prelude::EventWriter;

use crate::build::EnablePlayMenu;
pub const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.45);
pub const PRESSED_BUTTON: Color = Color::rgb(0.49, 0.73, 0.91);
use crate::build::MainMenuStarWolvesLink;

use crate::build::SpaceFrontiersHeader;
use crate::build::STARWOLVES_TEXT_COLOR;
use bevy::text::Text;

pub const SPACE_FRONTIERS_HEADER_TEXT_COLOR: Color = Color::rgb(0.00,0.84,1.00);

#[cfg(feature = "client")]
pub(crate) fn space_frontiers_link(
    mut interaction_query: Query<
        (&Interaction, &mut Text),
        (Changed<Interaction>, With<SpaceFrontiersHeader>),
    >,
) {
    for (interaction, mut text) in interaction_query.iter_mut() {
        let starwolves_text;

        match text.sections.get_mut(0) {
            Some(t) => {
                starwolves_text = t;
            }
            None => {
                warn!("Couldnt find space frontiers text section!");
                continue;
            }
        }

        match *interaction {
            Interaction::Clicked => {
                starwolves_text.style.color = Color::MIDNIGHT_BLUE.into();
                match open::that("http://github.com/starwolves/space") {
                    Ok(_) => {}
                    Err(_rr) => {
                        warn!("Couldn't open url http://github.com/starwolves/space !");
                    }
                }
            }
            Interaction::Hovered => {
                starwolves_text.style.color = Color::BLUE.into();
            }
            Interaction::None => {
                starwolves_text.style.color = SPACE_FRONTIERS_HEADER_TEXT_COLOR.into();
            }
        }
    }
}

#[cfg(feature = "client")]
pub(crate) fn starwolves_link(
    mut interaction_query: Query<
        (&Interaction, &mut Text),
        (Changed<Interaction>, With<MainMenuStarWolvesLink>),
    >,
) {
    for (interaction, mut text) in interaction_query.iter_mut() {
        let starwolves_text;

        match text.sections.get_mut(1) {
            Some(t) => {
                starwolves_text = t;
            }
            None => {
                warn!("Couldnt find starwolves text section!");
                continue;
            }
        }

        match *interaction {
            Interaction::Clicked => {
                starwolves_text.style.color = Color::BLUE.into();
                match open::that("http://starwolves.io") {
                    Ok(_) => {}
                    Err(_rr) => {
                        warn!("Couldn't open url https://starwolves.io !");
                    }
                }
            }
            Interaction::Hovered => {
                starwolves_text.style.color = Color::PINK.into();
            }
            Interaction::None => {
                starwolves_text.style.color = STARWOLVES_TEXT_COLOR.into();
            }
        }
    }
}

#[cfg(feature = "client")]
pub(crate) fn button_hover_visuals(
    mut interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    mut color_query: Query<&mut UiColor>,
) {
    for (entity, interaction, parent) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match color_query.get_mut(parent.get()) {
                    Ok(mut c) => {
                        *c = PRESSED_BUTTON.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button parent.");
                        continue;
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = PRESSED_BUTTON.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
            Interaction::Hovered => {
                match color_query.get_mut(parent.get()) {
                    Ok(mut c) => {
                        *c = HOVERED_BUTTON.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button parent.");
                        continue;
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = HOVERED_BUTTON.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
            Interaction::None => {
                match color_query.get_mut(parent.get()) {
                    Ok(mut c) => {
                        *c = SIDEBAR_COLOR.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button parent.");
                        continue;
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = SIDEBAR_COLOR.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
        }
    }
}
use crate::build::TextInputNode;

/// Manages text input UI nodes.
#[cfg(feature = "client")]
pub(crate) fn text_input_node_events(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &mut TextInputNode),
        Changed<Interaction>,
    >,
) {
    use crate::build::{INPUT_TEXT_BG, INPUT_TEXT_BG_HOVER, INPUT_TEXT_BG_PRESSED};

    for (interaction, mut color, text_input) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = INPUT_TEXT_BG_PRESSED.into();
            }
            Interaction::Hovered => {
                *color = INPUT_TEXT_BG_HOVER.into();
            }
            Interaction::None => {
                *color = INPUT_TEXT_BG.into();
            }
        }
    }
}
use bevy::app::AppExit;

#[cfg(feature = "client")]
pub(crate) fn button_presses(
    play_button_query: Query<
        (&Interaction, &MainMenuPlayButton),
        (Changed<Interaction>, With<Button>),
    >,
    settings_button_query: Query<
        (&Interaction, &MainMenuSettingsButton),
        (Changed<Interaction>, With<Button>),
    >,
    exit_button_query: Query<
        (&Interaction, &MainMenuExitButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut play_events: EventWriter<EnablePlayMenu>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, _) in &play_button_query {
        match *interaction {
            Interaction::Clicked => {
                play_events.send(EnablePlayMenu { enable: true });
            }
            _ => (),
        }
    }
    for (interaction, _) in &settings_button_query {
        match *interaction {
            Interaction::Clicked => {}
            _ => (),
        }
    }
    for (interaction, _) in &exit_button_query {
        match *interaction {
            Interaction::Clicked => {
                exit.send(AppExit);
            }
            _ => (),
        }
    }
}
