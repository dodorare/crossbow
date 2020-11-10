#![allow(dead_code)]

mod scenes;

use bevy::{
    app::ScheduleRunnerSettings,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    println!("Initialization.");
    App::build()
        .add_plugin(SetupPlugin)
        // .add_plugin(scenes::ScenesPlugin)
        .run();
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            // .add_resource(Msaa { samples: 4 })
            .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
            .add_resource(WindowDescriptor {
                title: "AppExample".to_string(),
                width: 340,
                height: 600,
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(text_update_system.system());
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &FpsText)>) {
    for (mut text, _tag) in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::RED,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}
