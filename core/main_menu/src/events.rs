use crate::build::MainMenuPlayButton;
use crate::build::{MainMenuExitButton, MainMenuSettingsButton};
use bevy::prelude::warn;
use bevy::prelude::{Changed, Color};
use bevy::{
    prelude::{Button, Query, With},
    ui::Interaction,
};

use bevy::prelude::EventWriter;

use crate::build::EnablePlayMenu;
use crate::build::MainMenuStarWolvesLink;

use crate::build::SpaceFrontiersHeader;
use crate::build::STARWOLVES_TEXT_COLOR;
use bevy::text::Text;

pub const SPACE_FRONTIERS_HEADER_TEXT_COLOR: Color = Color::rgb(0.46 * 1.6, 0.5 * 1.6, 0.69 * 1.6);

pub const SPACE_FRONTIERS_HEADER_TEXT_COLOR_HOVERED: Color =
    Color::rgb(0.46 * 1.9, 0.5 * 1.9, 0.69 * 1.9);

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
                starwolves_text.style.color = Color::BLUE.into();
                match open::that("http://github.com/starwolves/space") {
                    Ok(_) => {}
                    Err(_rr) => {
                        warn!("Couldn't open url http://github.com/starwolves/space !");
                    }
                }
            }
            Interaction::Hovered => {
                starwolves_text.style.color = SPACE_FRONTIERS_HEADER_TEXT_COLOR_HOVERED.into();
            }
            Interaction::None => {
                starwolves_text.style.color = SPACE_FRONTIERS_HEADER_TEXT_COLOR.into();
            }
        }
    }
}

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

/// Manages text input UI nodes.
use bevy::app::AppExit;

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
use crate::build::AccountNameInput;
use crate::build::ConnectToServerButton;
use bevy::prelude::Entity;
use networking::client::ConnectToServer;
use ui::text_input::TextInputNode;

/// Client connection preferences.
use bevy::prelude::ResMut;
use networking::client::ConnectionPreferences;

use crate::build::IpAddressInput;

pub(crate) fn connect_to_server_button(
    button_query: Query<(&ConnectToServerButton, &Interaction), Changed<Interaction>>,
    mut connect: EventWriter<ConnectToServer>,
    account_name_input_query: Query<(Entity, &AccountNameInput, &TextInputNode)>,
    server_address_input_query: Query<(Entity, &IpAddressInput, &TextInputNode)>,
    mut preferences: ResMut<ConnectionPreferences>,
) {
    for (_, interaction) in button_query.iter() {
        match interaction {
            Interaction::Clicked => {
                let mut account_name_input_entity_option = None;

                for (entity, _, _) in account_name_input_query.iter() {
                    account_name_input_entity_option = Some(entity);
                    break;
                }

                let mut server_address_input_entity_option = None;

                for (entity, _, _) in server_address_input_query.iter() {
                    server_address_input_entity_option = Some(entity);
                    break;
                }

                let account_name_input_entity;

                match account_name_input_entity_option {
                    Some(name) => {
                        account_name_input_entity = name;
                    }
                    None => {
                        warn!("Couldnt find account name input.");
                        continue;
                    }
                }

                let server_address_input_entity;

                match server_address_input_entity_option {
                    Some(address) => {
                        server_address_input_entity = address;
                    }
                    None => {
                        warn!("Couldnt find server address input.");
                        continue;
                    }
                }

                let account_name_node = account_name_input_query
                    .get_component::<TextInputNode>(account_name_input_entity)
                    .unwrap();
                let server_address_node = server_address_input_query
                    .get_component::<TextInputNode>(server_address_input_entity)
                    .unwrap();

                let account_name = account_name_node.input.clone();
                let server_address = server_address_node.input.clone();

                if account_name.len() < 3 {
                    warn!("Account name is too short.");
                    continue;
                }

                preferences.account_name = account_name;
                preferences.server_address = server_address;
                connect.send(ConnectToServer);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
