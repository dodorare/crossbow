mod button;

pub use button::*;

use bevy::{
    prelude::*, render::camera::ActiveCameras, render::render_graph::base::camera::CAMERA3D,
};

use super::state::*;

pub struct MenuScene;
impl Plugin for MenuScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_stage_after(stage::POST_UPDATE, "HANDLE_RUNSTATE")
            .init_resource::<ButtonMaterials>()
            .add_resource(RunState::new(GameState::Explorer))
            .add_startup_system(setup.system())
            .add_system(main_menu_setup.system())
            .add_system(threed_scene.system())
            .add_system(twod_scene.system())
            .add_system(explorer.system())
            .add_system(main_menu_button_system.system())
            .add_system(back_button_system.system())
            .add_system(state_despawn_system.system())
            .add_system(button_effect_system.system())
            .add_system_to_stage("HANDLE_RUNSTATE", run_state_fsm_system.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera2dComponents::default());
}

#[derive(Clone, Copy)]
enum MainMenuButton {
    ThreeDScene,
    TwoDScene,
    Audio,
    Explorer,
    About,
}

impl ToString for MainMenuButton {
    fn to_string(&self) -> String {
        match self {
            MainMenuButton::ThreeDScene => "3D Scene".to_owned(),
            MainMenuButton::TwoDScene => "2D Scene".to_owned(),
            MainMenuButton::Audio => "Audio".to_owned(),
            MainMenuButton::Explorer => "Explorer".to_owned(),
            MainMenuButton::About => "About".to_owned(),
        }
    }
}

fn main_menu_button_system(
    mut run_state: ResMut<RunState>,
    interaction_query: Query<(&Node, Mutated<Interaction>, &MainMenuButton)>,
) {
    for (_node, interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button {
                MainMenuButton::ThreeDScene => {
                    run_state.game_state.transit_to(GameState::ThreeDScene);
                }
                MainMenuButton::TwoDScene => {
                    run_state.game_state.transit_to(GameState::TwoDScene);
                }
                MainMenuButton::Audio => (),
                MainMenuButton::Explorer => {
                    run_state.game_state.transit_to(GameState::Explorer);
                }
                MainMenuButton::About => (),
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn spawn_main_menu_buttons(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    button_materials: &Res<ButtonMaterials>,
) {
    for button in &[
        MainMenuButton::ThreeDScene,
        MainMenuButton::TwoDScene,
        MainMenuButton::Audio,
        MainMenuButton::Explorer,
        MainMenuButton::About,
    ] {
        spawn_main_menu_button(parent, asset_server, &button_materials, *button);
    }
}

fn spawn_main_menu_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    button_materials: &Res<ButtonMaterials>,
    button: MainMenuButton,
) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Undefined,
                    bottom: Val::Px(20.0),
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextComponents {
                    text: Text {
                        value: button.to_string(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        style: TextStyle {
                            // FIXME: Should be fixed in bevy
                            font_size: if cfg!(target_os = "android") {
                                120.0
                            } else {
                                40.0
                            },
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    },
                    ..Default::default()
                })
                .with(ForStates {
                    states: vec![GameState::MainMenu],
                });
        })
        .with(ForStates {
            states: vec![GameState::MainMenu],
        })
        .with(button)
        .with(Interaction::default());
}

fn main_menu_setup(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if run_state.game_state.entering(GameState::MainMenu) {
        commands
            // root node (padding)
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    padding: Rect::all(Val::Percent(6.0)),
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    // main menu node
                    .spawn(NodeComponents {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        ..Default::default()
                    })
                    // main menu buttons
                    .with_children(|parent| {
                        spawn_main_menu_buttons(parent, &asset_server, &button_materials);
                    })
                    .with(ForStates {
                        states: vec![GameState::MainMenu],
                    });
            })
            .with(ForStates {
                states: vec![GameState::MainMenu],
            });
    };
}

struct BackButton;

fn back_button_system(
    mut run_state: ResMut<RunState>,
    interaction_query: Query<(&Node, Mutated<Interaction>, &BackButton)>,
) {
    for (_node, interaction, _button) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => run_state.game_state.transit_to(GameState::MainMenu),
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn spawn_back_button(
    commands: &mut Commands,
    texture_handle: Handle<Texture>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(
                    Val::Px(if cfg!(target_os = "android") {
                        210.0
                    } else {
                        70.0
                    }),
                    Val::Px(if cfg!(target_os = "android") {
                        210.0
                    } else {
                        70.0
                    }),
                ),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Undefined,
                    bottom: Val::Percent(10.0),
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            draw: Draw {
                is_transparent: true,
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: color_materials.add(texture_handle.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Undefined,
                                    bottom: Val::Undefined,
                                },
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            draw: Draw {
                                is_transparent: true,
                                is_visible: false,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(ForStates {
                            states: vec![
                                GameState::ThreeDScene,
                                GameState::TwoDScene,
                                GameState::Explorer,
                            ],
                        })
                        .with(BackButton)
                        .with(Interaction::default());
                })
                .with(ForStates {
                    states: vec![
                        GameState::ThreeDScene,
                        GameState::TwoDScene,
                        GameState::Explorer,
                    ],
                });
        })
        .with(ForStates {
            states: vec![
                GameState::ThreeDScene,
                GameState::TwoDScene,
                GameState::Explorer,
            ],
        });
}

fn threed_scene(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // FIXME: Should be fixed in bevy, cant despawn cameras
    mut active_cameras: ResMut<ActiveCameras>,
) {
    if run_state.game_state.entering(GameState::ThreeDScene) {
        // FIXME: Should be fixed in bevy, cant despawn cameras
        active_cameras.add(CAMERA3D);
        commands
            .spawn(Camera3dComponents {
                transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                    .looking_at(Vec3::default(), Vec3::unit_y()),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::ThreeDScene],
            })
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::ThreeDScene],
            })
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::ThreeDScene],
            })
            .spawn(LightComponents {
                transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::ThreeDScene],
            });

        let texture_handle = asset_server.load("branding/arrow_left.png");
        spawn_back_button(&mut commands, texture_handle, &mut color_materials);
    };
}

fn twod_scene(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if run_state.game_state.entering(GameState::TwoDScene) {
        let texture_handle = asset_server.load("branding/icon.png");
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::TwoDScene],
            });

        let texture_handle = asset_server.load("branding/arrow_left.png");
        spawn_back_button(&mut commands, texture_handle, &mut color_materials);
    }
}

fn explorer(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if run_state.game_state.entering(GameState::Explorer) {
        commands
            // root node (padding)
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    padding: Rect::all(Val::Percent(6.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    // explorer node
                    .spawn(NodeComponents {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::FlexStart,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        ..Default::default()
                    })
                    // explorer info block
                    .with_children(|parent| {
                        parent
                            .spawn(NodeComponents {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Auto),
                                    padding: Rect::all(Val::Percent(3.0)),
                                    flex_direction: FlexDirection::ColumnReverse,
                                    align_items: AlignItems::FlexStart,
                                    ..Default::default()
                                },
                                material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(TextComponents {
                                        style: Style {
                                            // Wrapping text dont work in bevy for now
                                            // flex_wrap: FlexWrap::Wrap,
                                            ..Default::default()
                                        },
                                        text: Text {
                                            value: "Height: 234242".to_string(),
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            style: TextStyle {
                                                // FIXME: Should be fixed in bevy
                                                font_size: if cfg!(target_os = "android") {
                                                    90.0
                                                } else {
                                                    30.0
                                                },
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        },
                                        ..Default::default()
                                    })
                                    .with(ForStates {
                                        states: vec![GameState::Explorer],
                                    })
                                    .spawn(TextComponents {
                                        text: Text {
                                            value: "best/finalize".to_string(),
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            style: TextStyle {
                                                // FIXME: Should be fixed in bevy
                                                font_size: if cfg!(target_os = "android") {
                                                    90.0
                                                } else {
                                                    30.0
                                                },
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        },
                                        ..Default::default()
                                    })
                                    .with(ForStates {
                                        states: vec![GameState::Explorer],
                                    })
                                    .spawn(TextComponents {
                                        text: Text {
                                            value: "Recent block: 1db...1fe".to_string(),
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            style: TextStyle {
                                                // FIXME: Should be fixed in bevy
                                                font_size: if cfg!(target_os = "android") {
                                                    90.0
                                                } else {
                                                    30.0
                                                },
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        },
                                        ..Default::default()
                                    })
                                    .with(ForStates {
                                        states: vec![GameState::Explorer],
                                    });
                            })
                            .with(ForStates {
                                states: vec![GameState::Explorer],
                            });
                    })
                    .with(ForStates {
                        states: vec![GameState::Explorer],
                    });

                // explorer node
                parent
                    .spawn(NodeComponents {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(12.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        material: materials.add(Color::RED.into()),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeComponents {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    margin: Rect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        top: Val::Undefined,
                                        bottom: Val::Undefined,
                                    },
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                material: button_materials.normal.clone(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(TextComponents {
                                        text: Text {
                                            value: "Refresh".to_string(),
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            style: TextStyle {
                                                // FIXME: Should be fixed in bevy
                                                font_size: if cfg!(target_os = "android") {
                                                    120.0
                                                } else {
                                                    40.0
                                                },
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        },
                                        ..Default::default()
                                    })
                                    .with(ForStates {
                                        states: vec![GameState::Explorer],
                                    });
                            })
                            .with(ForStates {
                                states: vec![GameState::Explorer],
                            })
                            .with(Interaction::default());
                    })
                    .with(ForStates {
                        states: vec![GameState::Explorer],
                    });
            })
            .with(ForStates {
                states: vec![GameState::Explorer],
            });
    }
}
