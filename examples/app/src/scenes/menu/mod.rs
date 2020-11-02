mod button;

pub use button::*;

use bevy::prelude::*;

use super::state::*;

pub struct MenuScene;
impl Plugin for MenuScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_stage_after(stage::POST_UPDATE, "HANDLE_RUNSTATE")
            .init_resource::<ButtonMaterials>()
            .add_resource(RunState::new(GameState::MainMenu))
            .add_system(main_menu_setup.system())
            .add_system(threed_scene.system())
            .add_system(twod_scene.system())
            .add_system(main_menu_button_system.system())
            .add_system(state_despawn_system.system())
            .add_system(button_effect_system.system())
            .add_system_to_stage("HANDLE_RUNSTATE", run_state_fsm_system.system());
    }
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
    mut interaction_query: Query<(&Node, Mutated<Interaction>, &MainMenuButton)>,
) {
    for (_node, interaction, button) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button {
                MainMenuButton::ThreeDScene => {
                    run_state.game_state.transit_to(GameState::ThreeDScene);
                }
                MainMenuButton::TwoDScene => {
                    run_state.game_state.transit_to(GameState::TwoDScene);
                }
                MainMenuButton::Audio => (),
                MainMenuButton::Explorer => (),
                MainMenuButton::About => (),
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn spawn_main_menu_buttons(
    parent: &mut ChildBuilder,
    font_handle: &Handle<Font>,
    button_materials: &Res<ButtonMaterials>,
) {
    for button in &[
        MainMenuButton::ThreeDScene,
        MainMenuButton::TwoDScene,
        MainMenuButton::Audio,
        MainMenuButton::Explorer,
        MainMenuButton::About,
    ] {
        spawn_main_menu_button(parent, font_handle.clone(), &button_materials, *button);
    }
}

fn spawn_main_menu_button(
    parent: &mut ChildBuilder,
    font_handle: Handle<Font>,
    button_materials: &Res<ButtonMaterials>,
    button: MainMenuButton,
) {
    parent
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
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
                        font: font_handle,
                        style: TextStyle {
                            font_size: 40.0,
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
        let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
        commands
            .spawn(UiCameraComponents::default())
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
                        material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                        ..Default::default()
                    })
                    // main menu buttons
                    .with_children(|parent| {
                        spawn_main_menu_buttons(parent, &font_handle, &button_materials);
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

fn threed_scene(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    asset_server: Res<AssetServer>,
) {
    if run_state.game_state.entering(GameState::ThreeDScene) {
        commands
            .spawn_scene(asset_server.load("models/helmet/FlightHelmet.gltf"))
            .spawn(LightComponents {
                transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::ThreeDScene],
            })
            .spawn(Camera3dComponents {
                transform: Transform::from_translation(Vec3::new(0.7, 0.7, 1.0))
                    .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::unit_y()),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::ThreeDScene],
            });
    };
}

fn twod_scene(
    mut commands: Commands,
    run_state: ResMut<RunState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if run_state.game_state.entering(GameState::TwoDScene) {
        let texture_handle = asset_server.load("branding/icon.png");
        commands
            .spawn(Camera2dComponents::default())
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with(ForStates {
                states: vec![GameState::TwoDScene],
            });
    }
}
