use bevy::text::TextSection;
use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Camera2dBundle, Color, Commands, Component,
        Entity, EventReader, EventWriter, NodeBundle, Res, ResMut, SystemLabel, TextBundle,
    },
    text::TextStyle,
    ui::{AlignItems, FlexDirection, FlexWrap, JustifyContent, Size, Style, UiRect, Val},
};
use resources::core::ClientInformation;

/// Event.
#[cfg(feature = "client")]
pub struct EnableMainMenu {
    pub enable: bool,
}

/// Resource containing the main menu state.
#[derive(Default, Resource)]
#[cfg(feature = "client")]
pub struct MainMenuState {
    pub enabled: bool,
    pub root: Option<Entity>,
    pub camera: Option<Entity>,
}

/// Labels for system ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "client")]
pub enum MainMenuLabel {
    BuildMainMenu,
}

/// Shows main menu when the client starts.
#[cfg(feature = "client")]
pub(crate) fn startup_show_menu(mut enable_events: EventWriter<EnableMainMenu>) {
    enable_events.send(EnableMainMenu { enable: true });
}

pub const SIDEBAR_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const STARWOLVES_TEXT_COLOR: Color = Color::VIOLET;
pub const TEXT_INPUT_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub const MAIN_BG_COLOR: Color = Color::DARK_GRAY;

#[derive(Component)]
#[cfg(feature = "client")]
pub(crate) struct MainMenuPlayButton;
#[derive(Component)]
#[cfg(feature = "client")]
pub(crate) struct MainMenuSettingsButton;
#[derive(Component)]
#[cfg(feature = "client")]
pub(crate) struct MainMenuExitButton;

#[derive(Component)]
#[cfg(feature = "client")]
pub(crate) struct SpaceFrontiersHeader;
use crate::events::SPACE_FRONTIERS_HEADER_TEXT_COLOR;

/// System that toggles the base visiblity of the main menu.
#[cfg(feature = "client")]
pub(crate) fn show_main_menu(
    mut enable_events: EventReader<EnableMainMenu>,
    mut state: ResMut<MainMenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    client_information: Res<ClientInformation>,
    mut show_play_menu: EventWriter<EnablePlayMenu>,
) {
    use ui::button::ButtonVisuals;

    if state.enabled {
        return;
    }

    for event in enable_events.iter() {
        if !event.enable {
            continue;
        }
        if state.enabled {
            continue;
        }

        state.enabled = true;

        // Open play menu by default.
        show_play_menu.send(EnablePlayMenu { enable: true });

        let camera_entity = commands.spawn(()).insert(Camera2dBundle::default()).id();

        state.camera = Some(camera_entity);

        let mut builder = commands.spawn(());

        let entity = builder.id();

        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");
        let empire_font = asset_server.load("fonts/AAbsoluteEmpire.ttf");
        let nesathoberyl_font = asset_server.load("fonts/Nesathoberyl.ttf");

        // Root node.
        builder
            .insert(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },

                background_color: MAIN_BG_COLOR.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                //Sidebar.
                parent
                    .spawn(())
                    .insert(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        background_color: SIDEBAR_COLOR.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        //Footer
                        parent
                            .spawn(())
                            .insert(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Undefined),
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(2.5),
                                    ),
                                    ..Default::default()
                                },
                                background_color: SIDEBAR_COLOR.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(())
                                    .insert(NodeBundle {
                                        style: Style {
                                            ..Default::default()
                                        },
                                        background_color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn(())
                                            .insert(TextBundle::from_sections([
                                                TextSection::new(
                                                    "© ",
                                                    TextStyle {
                                                        font: nesathoberyl_font,
                                                        font_size: 12.0,
                                                        color: TEXT_COLOR,
                                                    },
                                                ),
                                                TextSection::new(
                                                    "StarWolves",
                                                    TextStyle {
                                                        font: empire_font.clone(),
                                                        font_size: 12.0,
                                                        color: STARWOLVES_TEXT_COLOR,
                                                    },
                                                ),
                                            ]))
                                            .insert((
                                                Interaction::default(),
                                                MainMenuStarWolvesLink,
                                            ));
                                    });
                            });
                        //Body (contains header node)
                        parent
                            .spawn(())
                            .insert(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(10.),
                                        Val::Undefined,
                                    ),
                                    ..Default::default()
                                },
                                background_color: SIDEBAR_COLOR.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                // Header in body.
                                parent
                                    .spawn(())
                                    .insert(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.0), Val::Undefined),
                                            align_items: AlignItems::Center,
                                            flex_wrap: FlexWrap::Wrap,
                                            margin: UiRect::new(
                                                Val::Undefined,
                                                Val::Undefined,
                                                Val::Undefined,
                                                Val::Percent(25.),
                                            ),
                                            flex_direction: FlexDirection::ColumnReverse,
                                            ..Default::default()
                                        },
                                        background_color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(()).insert(TextBundle::from_section(
                                            "    ",
                                            TextStyle {
                                                font_size: 18.0,
                                                color: TEXT_COLOR,
                                                font: asset_server
                                                    .load("fonts/FontAwesome6Free-Solid-900.otf"),
                                            },
                                        ));
                                        parent.spawn(()).insert(
                                            TextBundle::from_section(
                                                client_information.version.clone(),
                                                TextStyle {
                                                    font_size: 11.0,
                                                    color: SPACE_FRONTIERS_HEADER_TEXT_COLOR,
                                                    font: arizone_font.clone(),
                                                },
                                            )
                                            .with_style(Style {
                                                margin: UiRect::new(
                                                    Val::Undefined,
                                                    Val::Undefined,
                                                    Val::Undefined,
                                                    Val::Percent(3.),
                                                ),
                                                ..Default::default()
                                            }),
                                        );
                                        parent
                                            .spawn(())
                                            .insert(
                                                TextBundle::from_section(
                                                    "SpaceFrontiers",
                                                    TextStyle {
                                                        font_size: 26.0,
                                                        color: SPACE_FRONTIERS_HEADER_TEXT_COLOR,
                                                        font: arizone_font.clone(),
                                                    },
                                                )
                                                .with_style(Style {
                                                    margin: UiRect::new(
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Percent(3.),
                                                    ),
                                                    ..Default::default()
                                                }),
                                            )
                                            .insert((Interaction::default(), SpaceFrontiersHeader));
                                    });
                                // Sidebar buttons.
                                parent
                                    .spawn(())
                                    .insert(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.0), Val::Undefined),
                                            align_items: AlignItems::Center,
                                            flex_wrap: FlexWrap::Wrap,
                                            flex_direction: FlexDirection::ColumnReverse,
                                            ..Default::default()
                                        },
                                        background_color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn(())
                                            .insert(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Undefined,
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    flex_wrap: FlexWrap::Wrap,
                                                    flex_direction: FlexDirection::ColumnReverse,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(())
                                                    .insert(ButtonBundle {
                                                        background_color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert((
                                                        MainMenuExitButton,
                                                        ButtonVisuals::default(),
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(()).insert(
                                                            TextBundle::from_section(
                                                                "Exit",
                                                                TextStyle {
                                                                    font: arizone_font.clone(),
                                                                    font_size: 20.0,
                                                                    color: TEXT_COLOR,
                                                                },
                                                            ),
                                                        );
                                                    });
                                            });
                                        parent
                                            .spawn(())
                                            .insert(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Undefined,
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    flex_wrap: FlexWrap::Wrap,
                                                    flex_direction: FlexDirection::ColumnReverse,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(())
                                                    .insert(ButtonBundle {
                                                        background_color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert((
                                                        MainMenuSettingsButton,
                                                        ButtonVisuals::default(),
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(()).insert(
                                                            TextBundle::from_section(
                                                                "Settings",
                                                                TextStyle {
                                                                    font: arizone_font.clone(),
                                                                    font_size: 20.0,
                                                                    color: TEXT_COLOR,
                                                                },
                                                            ),
                                                        );
                                                    });
                                            });
                                        parent
                                            .spawn(())
                                            .insert(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Undefined,
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    flex_wrap: FlexWrap::Wrap,
                                                    flex_direction: FlexDirection::ColumnReverse,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(())
                                                    .insert(ButtonBundle {
                                                        background_color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert((
                                                        MainMenuPlayButton,
                                                        ButtonVisuals::default(),
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(()).insert(
                                                            TextBundle::from_section(
                                                                "Play",
                                                                TextStyle {
                                                                    font: arizone_font.clone(),
                                                                    font_size: 20.0,
                                                                    color: TEXT_COLOR,
                                                                },
                                                            ),
                                                        );
                                                    });
                                            });
                                    });
                            });
                    });

                // Main
                parent
                    .spawn(())
                    .insert(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(75.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        background_color: MAIN_BG_COLOR.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // Root node of sub-main menus.
                        parent
                            .spawn(())
                            .insert(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    ..Default::default()
                                },
                                background_color: MAIN_BG_COLOR.into(),
                                ..Default::default()
                            })
                            .insert(MainMainMenuRoot);
                    });
            });

        state.root = Some(entity);
    }
}

pub const SUB_MENU_HEADER_COLOR: Color = INPUT_TEXT_BG;

#[derive(Component)]
#[cfg(feature = "client")]
pub struct MainMenuStarWolvesLink;

#[derive(Component)]
#[cfg(feature = "client")]
pub struct MainMainMenuRoot;

/// Event that enables play menu belonging to the main menu.
#[cfg(feature = "client")]
pub struct EnablePlayMenu {
    pub enable: bool,
}

/// Play menu state.
#[derive(Default, Resource)]
#[cfg(feature = "client")]
pub struct PlayMenuState {
    pub enabled: bool,
    pub root: Option<Entity>,
}
use bevy::prelude::Query;
use bevy::prelude::With;
use bevy::prelude::{warn, Resource};
use bevy::ui::AlignContent;
use bevy::ui::Interaction;

use ui::text_input::{CharacterFilter, TextInputNode, INPUT_TEXT_BG, INPUT_TEXT_BG_HOVER};

/// Displays play menu
#[cfg(feature = "client")]
pub(crate) fn show_play_menu(
    mut show_events: EventReader<EnablePlayMenu>,
    mut state: ResMut<PlayMenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_node_query: Query<Entity, With<MainMainMenuRoot>>,
) {
    use ui::button::{ButtonVisuals, HOVERED_BUTTON};

    for event in show_events.iter() {
        let mut root_node_option = None;
        for root in root_node_query.iter() {
            root_node_option = Some(root);
            break;
        }
        let root_node;
        match root_node_option {
            Some(n) => {
                root_node = n;
            }
            None => {
                warn!("Couldn't find root node!");
                continue;
            }
        }

        if !event.enable {
            continue;
        }
        if state.enabled {
            continue;
        }
        state.enabled = true;

        let entity = commands.spawn(()).id();

        commands.entity(root_node).add_child(entity);
        let mut builder = commands.entity(entity);
        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        builder
            .insert(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: MAIN_BG_COLOR.into(),
                ..Default::default()
            })
            // Menu.
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(60.), Val::Percent(60.)),
                            flex_wrap: FlexWrap::Wrap,
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        background_color: SIDEBAR_COLOR.into(),
                        ..Default::default()
                    })
                    // Play Body.
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(90.)),

                                    ..Default::default()
                                },
                                background_color: SIDEBAR_COLOR.into(),

                                ..Default::default()
                            })
                            .with_children(|parent| {
                                // Play Body container.
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            align_items: AlignItems::FlexEnd,
                                            align_content: AlignContent::FlexEnd,
                                            flex_wrap: FlexWrap::Wrap,
                                            justify_content: JustifyContent::Center,

                                            flex_direction: FlexDirection::ColumnReverse,
                                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                            ..Default::default()
                                        },
                                        background_color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    // Menu elements.
                                    .with_children(|parent| {
                                        // Connect button.
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.),
                                                        Val::Percent(5.),
                                                    ),
                                                    margin: UiRect::new(
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Percent(15.),
                                                        Val::Percent(10.),
                                                    ),
                                                    justify_content: JustifyContent::Center,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(ButtonBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Percent(100.),
                                                                Val::Percent(100.),
                                                            ),
                                                            justify_content: JustifyContent::Center,
                                                            align_items: AlignItems::Center,
                                                            flex_wrap: FlexWrap::Wrap,
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    })
                                                    .insert((
                                                        ButtonVisuals {
                                                            pressed_color: Color::BLUE,
                                                            default_color_option: Some(
                                                                HOVERED_BUTTON,
                                                            ),
                                                            default_parent_color: HOVERED_BUTTON,
                                                            hovered_color: INPUT_TEXT_BG_HOVER,
                                                            color_parent: false,
                                                            ..Default::default()
                                                        },
                                                        ConnectToServerButton,
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            "Connect",
                                                            TextStyle {
                                                                font: arizone_font.clone(),
                                                                font_size: 10.,
                                                                color: TEXT_INPUT_COLOR,
                                                            },
                                                        ));
                                                    });
                                            });
                                        // Input server IP.
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.),
                                                        Val::Percent(5.),
                                                    ),
                                                    justify_content: JustifyContent::Center,
                                                    margin: UiRect::new(
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Percent(7.),
                                                    ),
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                let text = "Enter address..";
                                                parent
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Percent(25.),
                                                                Val::Percent(100.),
                                                            ),
                                                            justify_content: JustifyContent::Center,
                                                            align_items: AlignItems::Center,
                                                            flex_wrap: FlexWrap::Wrap,
                                                            ..Default::default()
                                                        },
                                                        background_color: INPUT_TEXT_BG.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert((
                                                        TextInputNode {
                                                            placeholder_active: true,
                                                            character_filter_option: Some(
                                                                CharacterFilter::ServerAddress,
                                                            ),
                                                            placeholder_text_option: Some(
                                                                text.to_owned(),
                                                            ),
                                                            ..Default::default()
                                                        },
                                                        IpAddressInput,
                                                        Interaction::default(),
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            text,
                                                            TextStyle {
                                                                font: arizone_font.clone(),
                                                                font_size: 10.,
                                                                color: TEXT_INPUT_COLOR,
                                                            },
                                                        ));
                                                    });
                                            });
                                        // Label server ip.
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    margin: UiRect::new(
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Percent(3.),
                                                        Val::Percent(1.),
                                                    ),
                                                    size: Size::new(
                                                        Val::Percent(100.),
                                                        Val::Percent(5.),
                                                    ),
                                                    justify_content: JustifyContent::Center,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            justify_content: JustifyContent::Center,
                                                            ..Default::default()
                                                        },
                                                        background_color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            "IP address:",
                                                            TextStyle {
                                                                font: arizone_font.clone(),
                                                                font_size: 12.0,
                                                                color: TEXT_COLOR,
                                                            },
                                                        ));
                                                    });
                                            });
                                        // Input account name.
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.),
                                                        Val::Percent(5.),
                                                    ),
                                                    justify_content: JustifyContent::Center,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                let text = "Enter username..";
                                                parent
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            size: Size::new(
                                                                Val::Percent(25.),
                                                                Val::Percent(100.),
                                                            ),
                                                            justify_content: JustifyContent::Center,
                                                            align_items: AlignItems::Center,
                                                            flex_wrap: FlexWrap::Wrap,
                                                            ..Default::default()
                                                        },
                                                        background_color: INPUT_TEXT_BG.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert((
                                                        TextInputNode {
                                                            placeholder_active: true,
                                                            character_filter_option: Some(
                                                                CharacterFilter::AccountName,
                                                            ),
                                                            placeholder_text_option: Some(
                                                                text.to_owned(),
                                                            ),
                                                            ..Default::default()
                                                        },
                                                        AccountNameInput,
                                                        Interaction::default(),
                                                    ))
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            text,
                                                            TextStyle {
                                                                font: arizone_font.clone(),
                                                                font_size: 10.,
                                                                color: TEXT_INPUT_COLOR,
                                                            },
                                                        ));
                                                    });
                                            });
                                        // Label account name.
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    margin: UiRect::new(
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Undefined,
                                                        Val::Percent(1.),
                                                    ),

                                                    size: Size::new(
                                                        Val::Percent(100.),
                                                        Val::Percent(5.),
                                                    ),
                                                    justify_content: JustifyContent::Center,
                                                    ..Default::default()
                                                },
                                                background_color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            margin: UiRect::new(
                                                                Val::Undefined,
                                                                Val::Undefined,
                                                                Val::Undefined,
                                                                Val::Percent(1.),
                                                            ),
                                                            justify_content: JustifyContent::Center,
                                                            ..Default::default()
                                                        },
                                                        background_color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .with_children(|parent| {
                                                        parent.spawn(TextBundle::from_section(
                                                            "Account name:",
                                                            TextStyle {
                                                                font: arizone_font.clone(),
                                                                font_size: 12.0,
                                                                color: TEXT_COLOR,
                                                            },
                                                        ));
                                                    });
                                            });
                                    });
                            });
                        // Header.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                background_color: SUB_MENU_HEADER_COLOR.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        background_color: SUB_MENU_HEADER_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            "Connect to server",
                                            TextStyle {
                                                font: arizone_font.clone(),
                                                font_size: 12.0,
                                                color: TEXT_COLOR,
                                            },
                                        ));
                                    });
                            });
                    });
            });
    }
}

#[cfg(feature = "client")]
#[derive(Component)]
pub struct AccountNameInput;
#[cfg(feature = "client")]
#[derive(Component)]
pub struct IpAddressInput;
#[cfg(feature = "client")]
#[derive(Component)]
pub struct ConnectToServerButton;