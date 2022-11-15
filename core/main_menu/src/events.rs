use crate::build::MainMenuPlayButton;
use crate::build::NORMAL_BUTTON;
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

pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.49, 0.73, 0.91);

#[cfg(feature = "client")]
pub(crate) fn hover_visuals(
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
                        *c = NORMAL_BUTTON.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button parent.");
                        continue;
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = NORMAL_BUTTON.into();
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
) {
    for (interaction, _) in &play_button_query {
        match *interaction {
            Interaction::Clicked => {}
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
            Interaction::Clicked => {}
            _ => (),
        }
    }
}
