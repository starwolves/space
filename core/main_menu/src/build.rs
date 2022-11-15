use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Camera2dBundle, Color, Commands, Component,
        Entity, EventReader, EventWriter, NodeBundle, Res, ResMut, TextBundle,
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
#[derive(Default)]
#[cfg(feature = "client")]
pub struct MainMenuState {
    pub enabled: bool,
    pub root: Option<Entity>,
    pub camera: Option<Entity>,
}

/// Shows main menu when the client starts.
#[cfg(feature = "client")]
pub(crate) fn startup_show_menu(mut enable_events: EventWriter<EnableMainMenu>) {
    enable_events.send(EnableMainMenu { enable: true });
}

pub const SIDEBAR_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

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

/// System that toggles the base visiblity of the main menu.
#[cfg(feature = "client")]
pub(crate) fn show_main_menu(
    mut enable_events: EventReader<EnableMainMenu>,
    mut state: ResMut<MainMenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    client_information: Res<ClientInformation>,
) {
    use bevy::text::TextSection;

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

        let camera_entity = commands
            .spawn()
            .insert_bundle(Camera2dBundle::default())
            .id();

        state.camera = Some(camera_entity);

        let mut builder = commands.spawn();

        let entity = builder.id();

        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");
        let empire_font = asset_server.load("fonts/AAbsoluteEmpire.ttf");
        let nesathoberyl_font = asset_server.load("fonts/Nesathoberyl.ttf");

        // Root node.
        builder
            .insert_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: MAIN_BG_COLOR.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                //Sidebar.
                parent
                    .spawn()
                    .insert_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        color: SIDEBAR_COLOR.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        //Footer
                        parent
                            .spawn()
                            .insert_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(2.5),
                                    ),
                                    ..Default::default()
                                },
                                color: SIDEBAR_COLOR.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn()
                                    .insert_bundle(NodeBundle {
                                        style: Style {
                                            ..Default::default()
                                        },
                                        color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn().insert_bundle(TextBundle::from_sections([
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
                                                    color: TEXT_COLOR,
                                                },
                                            ),
                                        ]));
                                    });
                            });
                        //Body (contains header node)
                        parent
                            .spawn()
                            .insert_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                    flex_direction: FlexDirection::ColumnReverse,
                                    padding: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(13.),
                                        Val::Undefined,
                                    ),
                                    ..Default::default()
                                },
                                color: SIDEBAR_COLOR.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                // Header in body.
                                parent
                                    .spawn()
                                    .insert_bundle(NodeBundle {
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
                                            flex_direction: FlexDirection::Column,
                                            ..Default::default()
                                        },
                                        color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn().insert_bundle(TextBundle::from_section(
                                            "    ",
                                            TextStyle {
                                                font_size: 18.0,
                                                color: TEXT_COLOR,
                                                font: asset_server
                                                    .load("fonts/FontAwesome6Free-Solid-900.otf"),
                                            },
                                        ));
                                        parent.spawn().insert_bundle(
                                            TextBundle::from_section(
                                                client_information.version.clone(),
                                                TextStyle {
                                                    font_size: 11.0,
                                                    color: TEXT_COLOR,
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
                                        parent.spawn().insert_bundle(
                                            TextBundle::from_section(
                                                "SpaceFrontiers",
                                                TextStyle {
                                                    font_size: 26.0,
                                                    color: TEXT_COLOR,
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
                                    });
                                // Sidebar buttons.
                                parent
                                    .spawn()
                                    .insert_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.0), Val::Undefined),
                                            align_items: AlignItems::Center,
                                            flex_wrap: FlexWrap::Wrap,
                                            flex_direction: FlexDirection::Column,
                                            ..Default::default()
                                        },
                                        color: SIDEBAR_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn()
                                            .insert_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Undefined,
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    flex_wrap: FlexWrap::Wrap,
                                                    flex_direction: FlexDirection::Column,
                                                    ..Default::default()
                                                },
                                                color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn()
                                                    .insert_bundle(ButtonBundle {
                                                        color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert(MainMenuExitButton)
                                                    .with_children(|parent| {
                                                        parent.spawn().insert_bundle(
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
                                            .spawn()
                                            .insert_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Undefined,
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    flex_wrap: FlexWrap::Wrap,
                                                    flex_direction: FlexDirection::Column,
                                                    ..Default::default()
                                                },
                                                color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn()
                                                    .insert_bundle(ButtonBundle {
                                                        color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert(MainMenuSettingsButton)
                                                    .with_children(|parent| {
                                                        parent.spawn().insert_bundle(
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
                                            .spawn()
                                            .insert_bundle(NodeBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Undefined,
                                                    ),
                                                    align_items: AlignItems::Center,
                                                    flex_wrap: FlexWrap::Wrap,
                                                    flex_direction: FlexDirection::Column,
                                                    ..Default::default()
                                                },
                                                color: SIDEBAR_COLOR.into(),
                                                ..Default::default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn()
                                                    .insert_bundle(ButtonBundle {
                                                        color: SIDEBAR_COLOR.into(),
                                                        ..Default::default()
                                                    })
                                                    .insert(MainMenuPlayButton)
                                                    .with_children(|parent| {
                                                        parent.spawn().insert_bundle(
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
                    .spawn()
                    .insert_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(75.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        color: MAIN_BG_COLOR.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // Root node of sub-main menus.
                        parent
                            .spawn()
                            .insert_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    ..Default::default()
                                },
                                color: MAIN_BG_COLOR.into(),
                                ..Default::default()
                            })
                            .insert(MainMainMenuRoot);
                    });
            });

        state.root = Some(entity);
    }
}

#[derive(Component)]
#[cfg(feature = "client")]
pub struct MainMainMenuRoot;

/// Event that enables play menu belonging to the main menu.
#[cfg(feature = "client")]
pub struct EnablePlayMenu {
    pub enable: bool,
}

/// Play menu state.
#[derive(Default)]
#[cfg(feature = "client")]
pub struct PlayMenuState {
    pub enabled: bool,
    pub root: Option<Entity>,
}
use bevy::prelude::Query;
use bevy::prelude::With;

/// Displays play menu
#[cfg(feature = "client")]
pub(crate) fn show_play_menu(
    mut show_events: EventReader<EnablePlayMenu>,
    mut state: ResMut<PlayMenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_node_query: Query<Entity, With<MainMainMenuRoot>>,
) {
    use bevy::{prelude::warn, ui::AlignContent};

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

        let entity = commands.spawn().id();

        commands.entity(root_node).add_child(entity);
        let mut builder = commands.entity(entity);
        //let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        builder
            .insert_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: MAIN_BG_COLOR.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn().insert_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.), Val::Percent(60.)),
                        ..Default::default()
                    },
                    color: SIDEBAR_COLOR.into(),
                    ..Default::default()
                });
            });
    }
}
