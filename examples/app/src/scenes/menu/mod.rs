mod button;

pub use button::*;

use bevy::prelude::*;

pub struct MenuScene;
impl Plugin for MenuScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_resource::<ButtonMaterials>()
            .add_startup_system(menu_setup.system())
            .add_system(main_menu_buttons_system.system());
        // .add_system(button_system.system());
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Screen {
    MainMenu,
    ThreeDScene,
    TwoDScene,
    Audio,
    Explorer,
    About,
}

struct ScreenState {
    current_screen: Screen,
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

fn main_menu_buttons_system(
    // mut screen_state: ResMut<ScreenState>,
    mut interaction_query: Query<(&Node, Mutated<Interaction>, &MainMenuButton)>,
) {
    for (_node, interaction, button) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => println!("Clicked {}", button.to_string()),
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
        spawn_main_menu_button(parent, &asset_server, &button_materials, *button);
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
        .with(button)
        .with(Interaction::default())
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: button.to_string(),
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
        });
}

fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // ui camera
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
                    spawn_main_menu_buttons(parent, &asset_server, &button_materials);
                });
        });
}
