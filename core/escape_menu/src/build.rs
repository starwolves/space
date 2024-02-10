use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        BuildChildren, ButtonBundle, Color, Commands, Component, Entity, Label, NodeBundle, Res,
        TextBundle,
    },
    render::view::Visibility,
    text::TextStyle,
    ui::{
        AlignItems, Display, FlexDirection, FlexWrap, Interaction, JustifyContent, Overflow, Style,
        UiRect, Val,
    },
};
use graphics::settings::PerformanceSettings;
use resources::{hud::EscapeMenuState, input::KeyBinds};
use ui::{
    button::SFButton,
    fonts::{Fonts, ARIZONE_FONT, EMPIRE_FONT, SOURCECODE_REGULAR_FONT},
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
pub struct PerformanceSection;
#[derive(Component)]
pub struct PerformanceBGSection;

#[derive(Component)]
pub struct ControlsHeaderButton;
#[derive(Component)]
pub struct GeneralHeaderButton;
#[derive(Component)]
pub struct PerformanceHeaderButton;

pub const ESC_MENU_FONT_COLOR: Color = Color::WHITE;
pub const ESC_MENU_FONT_SIZE: f32 = 15.;

pub(crate) fn build_escape_menu(mut commands: Commands, fonts: Res<Fonts>) {
    let arizone_font = fonts.handles.get(ARIZONE_FONT).unwrap();
    let empire_font = fonts.handles.get(EMPIRE_FONT).unwrap();
    let mut controls_bg_section_entity = Entity::from_bits(0);
    let mut graphics_bg_section_entity = Entity::from_bits(0);
    let mut general_section_entity = Entity::from_bits(0);
    let mut controls_section = Entity::from_bits(0);
    let mut graphics_section_entity = Entity::from_bits(0);

    let escape_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
                        width: Val::Percent(50.0),
                        height: Val::Percent(60.0),
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
                                width: Val::Percent(100.0),
                                height: Val::Percent(7.0),
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
                                        width: Val::Percent(33.333333),
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(100.),
                                                height: Val::Percent(100.0),
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
                                                    font: arizone_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(33.333333),
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
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
                                                    font: arizone_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(33.333333),
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(100.),
                                                height: Val::Percent(100.),
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(SFButton::default())
                                        .insert(PerformanceHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Performance".to_string(),
                                                TextStyle {
                                                    font: arizone_font.clone(),
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
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::clip(),

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
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
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
                                .insert(PerformanceSection)
                                .id();
                        })
                        .insert(PerformanceBGSection)
                        .id();
                    general_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
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
                                    width: Val::Percent(100.),
                                    height: Val::Percent(10.),
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
                                        width: Val::Percent(100.),
                                        height: Val::Percent(10.),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Press ESCAPE to hide menu".to_string(),
                                        TextStyle {
                                            font: empire_font.clone(),
                                            font_size: 12.0,
                                            color: ESC_MENU_FONT_COLOR.into(),
                                        },
                                    ));
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(6.),
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
                                                width: Val::Percent(35.),
                                                height: Val::Percent(100.),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(ButtonBundle {
                                                    style: Style {
                                                        width: Val::Percent(100.),
                                                        height: Val::Percent(100.),
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
                                                            font: arizone_font.clone(),
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
    fonts: Res<Fonts>,
    state: Res<EscapeMenuState>,
    binds: Res<KeyBinds>,
) {
    let source_code = fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap();
    let font = fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap();

    commands
        .entity(state.controls_section)
        .with_children(|parent| {
            for (bind_id, bind) in binds.list.iter() {
                if !bind.customizable {
                    continue;
                }
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Auto,
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            padding: UiRect::left(Val::Percent(2.5)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert((Label, AccessibilityNode(NodeBuilder::new(Role::ListItem))))
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    bind.name.clone() + ": ",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: ESC_MENU_FONT_SIZE,
                                        color: ESC_MENU_FONT_COLOR.into(),
                                    },
                                ));
                                parent.spawn(TextBundle::from_section(
                                    bind.description.clone() + " ",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: ESC_MENU_FONT_SIZE,
                                        color: ESC_MENU_FONT_COLOR.into(),
                                    },
                                ));
                            });
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(10.),
                                    height: Val::Percent(100.),
                                    flex_direction: FlexDirection::RowReverse,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..Default::default()
                                },
                                background_color: Color::WHITE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.),
                                            height: Val::Auto,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn(ButtonBundle {
                                                style: Style {
                                                    width: Val::Percent(100.),
                                                    height: Val::Auto,
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
                                                        font_size: ESC_MENU_FONT_SIZE,
                                                        color: Color::WHITE.into(),
                                                    },
                                                ));
                                            });
                                    });
                            });
                    });
            }
        });
}

#[derive(Component)]
pub struct ResolutionXInput;
#[derive(Component)]
pub struct ResolutionYInput;
#[derive(Component)]
pub struct WindowModeHList;
#[derive(Component)]
pub struct VsyncHList;
#[derive(Component)]
pub struct SyncCorrectionHList;
#[derive(Component)]
pub struct ShadowsHList;
#[derive(Component)]
pub struct SyncCorrectionRestartLabel;
#[derive(Component)]
pub struct AmbientLightingRestartLabel;
#[derive(Component)]
pub struct RCASHList;
#[derive(Component)]
pub struct AmbientLightingHList;
#[derive(Component)]
pub struct FxaaHList;
#[derive(Component)]
pub struct MsaaHList;
#[derive(Component)]
pub struct ResolutionInputApply;

pub(crate) fn build_graphics_section(
    mut commands: Commands,
    fonts: Res<Fonts>,
    state: Res<EscapeMenuState>,
    settings: Res<PerformanceSettings>,
) {
    let source_code = fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap();
    let font = fonts.handles.get(SOURCECODE_REGULAR_FONT).unwrap();

    commands
        .entity(state.graphics_section)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        flex_wrap: FlexWrap::Wrap,
                        padding: UiRect::new(
                            Val::Percent(2.5),
                            Val::Px(0.),
                            Val::Percent(2.5),
                            Val::Px(0.),
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
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
                                },
                            ));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(5.),
                                        height: Val::Auto,
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
                                    ResolutionXInput,
                                    Interaction::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        settings.resolution.0.to_string(),
                                        TextStyle {
                                            font: source_code.clone(),
                                            font_size: ESC_MENU_FONT_SIZE,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(5.),
                                        height: Val::Auto,
                                        margin: UiRect::left(Val::Percent(0.6)),
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
                                    ResolutionYInput,
                                    Interaction::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        settings.resolution.1.to_string(),
                                        TextStyle {
                                            font: source_code.clone(),
                                            font_size: ESC_MENU_FONT_SIZE,
                                            color: Color::WHITE.into(),
                                        },
                                    ));
                                });

                            parent
                                .spawn(NodeBundle {
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                margin: UiRect::left(Val::Percent(0.6)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,

                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(SFButton::default())
                                        .insert(ResolutionInputApply)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Apply",
                                                TextStyle {
                                                    font: source_code.clone(),
                                                    font_size: ESC_MENU_FONT_SIZE,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
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
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                })
                                .insert(WindowModeHList);
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
                                "Synchronous physics (unfinished, causes jitter): ",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                    selected: Some(settings.synchronous_correction as u8),
                                    selections: vec!["Off".to_string(), "On".to_string()],
                                    ..Default::default()
                                })
                                .insert(SyncCorrectionHList);
                            parent
                                .spawn(TextBundle::from_section(
                                    " Restart required.",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: ESC_MENU_FONT_SIZE,
                                        color: Color::ORANGE_RED.into(),
                                    },
                                ))
                                .insert((SyncCorrectionRestartLabel, Visibility::Hidden));
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
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                })
                                .insert(VsyncHList);
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
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                })
                                .insert(FxaaHList);
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
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                })
                                .insert(MsaaHList);
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
                                "RCAS: ",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                    selected: Some(settings.rcas as u8),
                                    selections: vec!["Off".to_string(), "On".to_string()],
                                    ..Default::default()
                                })
                                .insert(RCASHList);
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
                                "Shadows: ",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                    selected: Some(settings.shadows.clone() as u8),
                                    selections: vec![
                                        "Off".to_string(),
                                        "Medium".to_string(),
                                        "High".to_string(),
                                    ],
                                    ..Default::default()
                                })
                                .insert(ShadowsHList);
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
                                "Ambient lighting: ",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: ESC_MENU_FONT_SIZE,
                                    color: ESC_MENU_FONT_COLOR.into(),
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
                                    selected: Some(settings.ambient_lighting as u8),
                                    selections: vec!["Off".to_string(), "On".to_string()],
                                    ..Default::default()
                                })
                                .insert(AmbientLightingHList);
                            parent
                                .spawn(TextBundle::from_section(
                                    " Restart required if turning off.",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: ESC_MENU_FONT_SIZE,
                                        color: Color::ORANGE_RED.into(),
                                    },
                                ))
                                .insert((AmbientLightingRestartLabel, Visibility::Hidden));
                        });
                });
        });
}
