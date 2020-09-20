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
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
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

// use bevy::{
//     diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
//     prelude::*,
//     asset::{AssetLoader, AssetLoadError},
//     text::FontAtlasSet,
// };

// .add_default_plugins()
// .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
// .add_plugin(TextPlugin::default())
// .add_plugin(FrameTimeDiagnosticsPlugin::default())
// .add_startup_system(setup.system())
// .add_system(text_update_system.system())
// .run();

// #[derive(Default)]
// pub struct FontLoader;

// impl AssetLoader<Font> for FontLoader {
//     fn from_bytes(&self, _asset_path: &std::path::Path, bytes: Vec<u8>) -> anyhow::Result<Font> {
//         Ok(Font::try_from_bytes(bytes)?)
//     }

//     fn extensions(&self) -> &[&str] {
//         static EXTENSIONS: &[&str] = &["ttf"];
//         EXTENSIONS
//     }

//     fn load_from_file(&self, asset_path: &std::path::Path) -> Result<Font, AssetLoadError> {
//         let mut bytes = Vec::new();
//         #[cfg(target_os = "android")]
//         {
//             use std::ffi::CString;
//             let na = ndk_glue::native_activity();
//             let mut font = na
//                 .asset_manager()
//                 .open(&CString::new(asset_path.to_str().unwrap()).unwrap())
//                 .unwrap();
//             let buf = font
//                 .get_buffer()
//                 .unwrap();
//             println!("{:?}", buf.len());
//             bytes.extend(buf.to_vec());
//         }
//         #[cfg(not(target_os = "android"))]
//         {
//             use std::{fs::File, io::prelude::*};
//             let mut file = File::open(asset_path)?;
//             file.read_to_end(&mut bytes)?;
//         }
//         let asset = self.from_bytes(asset_path, bytes)?;
//         Ok(asset)
//     }
// }

// #[derive(Default)]
// pub struct TextPlugin;

// impl Plugin for TextPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_asset::<Font>()
//             .add_asset::<FontAtlasSet>()
//             .add_asset_loader::<Font, FontLoader>();
//     }
// }

// fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
//     for mut text in &mut query.iter() {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(average) = fps.average() {
//                 text.value = format!("FPS: {:.2}", average);
//             }
//         }
//     }
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut fonts: ResMut<Assets<Font>>) {
//     let font_handle = asset_server.load_sync(&mut fonts, "fonts/FiraSans-Bold.ttf").unwrap();
//     commands
//         // 2d camera
//         .spawn(UiCameraComponents::default())
//         // texture
//         .spawn(TextComponents {
//             style: Style {
//                 align_self: AlignSelf::Center,
//                 ..Default::default()
//             },
//             text: Text {
//                 value: "FPS:".to_string(),
//                 font: font_handle,
//                 style: TextStyle {
//                     font_size: 60.0,
//                     color: Color::WHITE,
//                 },
//             },
//             ..Default::default()
//         });
// }
