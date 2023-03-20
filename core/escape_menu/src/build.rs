use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Color, Commands, Component, Entity, Label,
        NodeBundle, Res, Resource, TextBundle,
    },
    text::TextStyle,
    ui::{
        AlignItems, Display, FlexDirection, FlexWrap, Interaction, JustifyContent, Overflow, Size,
        Style, UiRect, Val,
    },
};
use graphics::settings::GraphicsSettings;
use resources::binds::KeyBinds;
use ui::{
    button::SFButton,
    fonts::{ARIZONE_FONT, SOURCECODE_REGULAR_FONT},
    hlist::HList,
    scrolling::ScrollingList,
    text_input::{CharacterFilter, TextInputNode},
};

#[derive(Component)]
pub struct EscapeMenuRoot;
#[derive(Component)]
pub struct GeneralSection;
#[derive(Component)]
pub struct ControlsSection;
#[derive(Component)]
pub struct ControlsBGSection;
#[derive(Component)]
pub struct GraphicsSection;
#[derive(Component)]
pub struct GraphicsBGSection;

#[derive(Resource)]
pub struct EscapeMenuState {
    pub root: Entity,
    pub visible: bool,
    pub controls_section: Entity,
    pub controls_bg_section: Entity,

    pub graphics_section: Entity,
    pub graphics_bg_section: Entity,

    pub general_section: Entity,
}
#[derive(Component)]
pub struct ControlsHeaderButton;
#[derive(Component)]
pub struct GeneralHeaderButton;
#[derive(Component)]
pub struct GraphicsHeaderButton;

pub(crate) fn build_escape_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sourcecode_font = asset_server.load(ARIZONE_FONT);
    let mut controls_bg_section_entity = Entity::from_bits(0);
    let mut graphics_bg_section_entity = Entity::from_bits(0);
    let mut general_section_entity = Entity::from_bits(0);
    let mut controls_section = Entity::from_bits(0);
    let mut graphics_section_entity = Entity::from_bits(0);

    let escape_root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EscapeMenuRoot)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(60.0)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.6, 0.73, 1., 0.6).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(7.)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_wrap: FlexWrap::Wrap,
                                ..Default::default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(33.333333),
                                            Val::Percent(100.),
                                        ),
                                        ..Default::default()
                                    },
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
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(SFButton::default())
                                        .insert(ControlsHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Controls".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(33.333333),
                                            Val::Percent(100.),
                                        ),
                                        ..Default::default()
                                    },
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
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(SFButton::default())
                                        .insert(GeneralHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "General".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(33.333333),
                                            Val::Percent(100.),
                                        ),
                                        ..Default::default()
                                    },
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
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(SFButton::default())
                                        .insert(GraphicsHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Graphics".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                        });
                    controls_bg_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::Hidden,

                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            controls_section = parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },))
                                .insert((
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .insert(ControlsSection)
                                .id();
                        })
                        .insert(ControlsBGSection)
                        .id();
                    graphics_bg_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            graphics_section_entity = parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },))
                                .insert((
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .insert(GraphicsSection)
                                .id();
                        })
                        .insert(GraphicsBGSection)
                        .id();
                    general_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GeneralSection)
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Percent(6.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(35.),
                                                    Val::Percent(100.),
                                                ),
                                                ..Default::default()
                                            },
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
                                                        flex_direction: FlexDirection::Row,
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        flex_wrap: FlexWrap::Wrap,
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                })
                                                .insert(SFButton::default())
                                                .insert(ExitGameButton)
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        "Exit Game".to_string(),
                                                        TextStyle {
                                                            font: sourcecode_font.clone(),
                                                            font_size: 12.0,
                                                            color: Color::WHITE.into(),
                                                        },
                                                    ));
                                                });
                                        });
                                });
                        })
                        .id();
                });
        })
        .id();
    commands.insert_resource(EscapeMenuState {
        root: escape_root,
        visible: false,
        controls_section,
        controls_bg_section: controls_bg_section_entity,
        graphics_section: graphics_section_entity,
        general_section: general_section_entity,
        graphics_bg_section: graphics_bg_section_entity,
    })
}
#[derive(Component)]
pub struct ExitGameButton;
#[derive(Component)]
pub struct BindButton {
    pub bind_id: String,
}

pub(crate) fn build_controls_section(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<EscapeMenuState>,
    binds: Res<KeyBinds>,
) {
    let source_code = asset_server.load(SOURCECODE_REGULAR_FONT);
    commands
        .entity(state.controls_section)
        .with_children(|parent| {
            for (bind_id, bind) in binds.list.iter() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Auto),
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            padding: UiRect::left(Val::Percent(2.5)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert((Label, AccessibilityNode(NodeBuilder::new(Role::ListItem))))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            bind.name.clone() + ": ",
                            TextStyle {
                                font: source_code.clone(),
                                font_size: 12.0,
                                color: Color::WHITE.into(),
                            },
                        ));
                        parent.spawn(TextBundle::from_section(
                            bind.description.clone(),
                            TextStyle {
                                font: source_code.clone(),
                                font_size: 12.0,
                                color: Color::WHITE.into(),
                            },
                        ));
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(10.), Val::Auto),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Auto),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .insert(SFButton::default())
                                    .insert(BindButton {
                                        bind_id: bind_id.clone(),
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            format!("{:?}", bind.key_code),
                                            TextStyle {
                                                font: source_code.clone(),
                                                font_size: 12.0,
                                                color: Color::WHITE.into(),
                                            },
                                        ));
                                    });
                            });
                    });
            }
        });
}

pub(crate) fn build_graphics_section(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<EscapeMenuState>,
    settings: Res<GraphicsSettings>,
) {
    let source_code = asset_server.load(SOURCECODE_REGULAR_FONT);

    commands
        .entity(state.graphics_section)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        flex_direction: FlexDirection::Column,
                        flex_wrap: FlexWrap::Wrap,
                        padding: UiRect::new(
                            Val::Percent(2.5),
                            Val::Undefined,
                            Val::Percent(2.5),
                            Val::Undefined,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert((Label, AccessibilityNode(NodeBuilder::new(Role::ListItem))))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resolution: ",
                                TextStyle {
                                    font: source_code.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE.into(),
                                },
                            ));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(5.), Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert((
                                    TextInputNode {
                                        placeholder_active: true,
                                        character_filter_option: Some(CharacterFilter::Integer),
                                        placeholder_text_option: Some(
                                            settings.resolution.0.to_string(),
                                        ),
                                        ..Default::default()
                                    },
                                    Interaction::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        settings.resolution.0.to_string(),
                                        TextStyle {
                                            font: source_code.clone(),
                                            font_size: 12.,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(5.), Val::Auto),
                                        margin: UiRect::left(Val::Percent(1.5)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,

                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert((
                                    TextInputNode {
                                        placeholder_active: true,
                                        character_filter_option: Some(CharacterFilter::Integer),
                                        placeholder_text_option: Some(
                                            settings.resolution.1.to_string(),
                                        ),
                                        ..Default::default()
                                    },
                                    Interaction::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        settings.resolution.1.to_string(),
                                        TextStyle {
                                            font: source_code.clone(),
                                            font_size: 12.,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Window Mode: ",
                                TextStyle {
                                    font: source_code.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE.into(),
                                },
                            ));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(HList {
                                    selected: Some(settings.window_mode.clone() as u8),
                                    selections: vec![
                                        "Windowed".to_string(),
                                        "Borderless Fullscreen".to_string(),
                                        "Sized Fullscreen".to_string(),
                                        "Fullscreen".to_string(),
                                    ],
                                    ..Default::default()
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Vsync: ",
                                TextStyle {
                                    font: source_code.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE.into(),
                                },
                            ));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(HList {
                                    selected: Some(settings.vsync as u8),
                                    selections: vec!["Off".to_string(), "On".to_string()],
                                    ..Default::default()
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Fxaa: ",
                                TextStyle {
                                    font: source_code.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE.into(),
                                },
                            ));
                            let selected_i;
                            match settings.fxaa.clone() {
                                Some(fx) => {
                                    selected_i = fx as u8 + 1;
                                }
                                None => {
                                    selected_i = 0;
                                }
                            }
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(HList {
                                    selected: Some(selected_i),
                                    selections: vec![
                                        "Off".to_string(),
                                        "Low".to_string(),
                                        "Medium".to_string(),
                                        "High".to_string(),
                                    ],
                                    ..Default::default()
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Msaa: ",
                                TextStyle {
                                    font: source_code.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE.into(),
                                },
                            ));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(HList {
                                    selected: Some(settings.msaa.clone() as u8),
                                    selections: vec![
                                        "Off".to_string(),
                                        "Low".to_string(),
                                        "Medium".to_string(),
                                        "High".to_string(),
                                    ],
                                    ..Default::default()
                                });
                        });
                });
        });
}
