#![allow(dead_code)]

#[cfg(target_os = "android")]
use android_logger::Config;

use bevy::prelude::*;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_min_level(log::Level::Trace));

    println!("The world!");
    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        // .add_startup_system(cube.system())
        .add_startup_system(icon.system())
        // .add_startup_system(text.system())
        .run();
}

/// set up a simple 3D scene
fn cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(-3.0, 3.0, 5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("branding/icon.png").unwrap();
    println!("texture_handle: {:?}", texture_handle);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

fn text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            text: Text {
                value: "Hello there".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 600.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        });
}
