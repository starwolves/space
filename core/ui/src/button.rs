use bevy::prelude::{Color, Component};

use crate::text_input::INPUT_TEXT_BG;

pub const HOVERED_BUTTON: Color = INPUT_TEXT_BG;
pub const PRESSED_BUTTON: Color = Color::rgb(0.49, 0.73, 0.91);

/// Component for button visuals.
#[cfg(feature = "client")]
#[derive(Component)]
pub struct ButtonVisuals {
    pub hovered_color: Color,
    pub pressed_color: Color,
    pub default_parent_color: Color,
    pub default_color_option: Option<Color>,
    pub color_parent: bool,
}

impl Default for ButtonVisuals {
    fn default() -> Self {
        Self {
            hovered_color: HOVERED_BUTTON,
            pressed_color: PRESSED_BUTTON,
            default_parent_color: Color::rgb(0.15, 0.15, 0.15),
            default_color_option: None,
            color_parent: true,
        }
    }
}
use bevy::prelude::warn;
use bevy::prelude::{Button, Parent, Query, With};
use bevy::ui::UiColor;
use bevy::{
    prelude::{Changed, Entity},
    ui::Interaction,
};

#[cfg(feature = "client")]
pub(crate) fn button_hover_visuals(
    mut interaction_query: Query<
        (Entity, &Interaction, &Parent, &ButtonVisuals),
        (Changed<Interaction>, With<Button>),
    >,
    mut color_query: Query<&mut UiColor>,
) {
    for (entity, interaction, parent, button_visuals) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if button_visuals.color_parent {
                    match color_query.get_mut(parent.get()) {
                        Ok(mut c) => {
                            *c = button_visuals.pressed_color.into();
                        }
                        Err(_rr) => {
                            warn!("Couldnt find button parent.");
                            continue;
                        }
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = button_visuals.pressed_color.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
            Interaction::Hovered => {
                if button_visuals.color_parent {
                    match color_query.get_mut(parent.get()) {
                        Ok(mut c) => {
                            *c = button_visuals.hovered_color.into();
                        }
                        Err(_rr) => {
                            warn!("Couldnt find button parent.");
                            continue;
                        }
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => {
                        *c = button_visuals.hovered_color.into();
                    }
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
            Interaction::None => {
                if button_visuals.color_parent {
                    match color_query.get_mut(parent.get()) {
                        Ok(mut c) => {
                            *c = button_visuals.default_parent_color.into();
                        }
                        Err(_rr) => {
                            warn!("Couldnt find button parent.");
                            continue;
                        }
                    }
                }
                match color_query.get_mut(entity) {
                    Ok(mut c) => match button_visuals.default_color_option {
                        Some(col) => {
                            *c = col.into();
                        }
                        None => {
                            *c = button_visuals.default_parent_color.into();
                        }
                    },
                    Err(_rr) => {
                        warn!("Couldnt find button.");
                        continue;
                    }
                }
            }
        }
    }
}
