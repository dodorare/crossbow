mod button;

pub use button::*;

use bevy::prelude::*;

pub struct UiScene;
impl Plugin for UiScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_resource::<ButtonMaterials>()
            .add_startup_system(ui_setup.system())
            .add_system(button_system.system());
    }
}

fn spawn_main_menu_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    button_materials: &Res<ButtonMaterials>,
    text: &str,
) {
    parent
        .spawn(ButtonComponents {
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
            parent.spawn(TextComponents {
                text: Text {
                    value: text.to_string(),
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

pub fn ui_setup(
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
                    spawn_main_menu_button(parent, &asset_server, &button_materials, "Button 1");
                    spawn_main_menu_button(parent, &asset_server, &button_materials, "Button 2");
                    spawn_main_menu_button(parent, &asset_server, &button_materials, "Button 3");
                });
        });
}
