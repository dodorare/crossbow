#![allow(dead_code)]

mod scenes;

use bevy::prelude::*;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    println!("Initialization.");
    App::build()
        .add_plugin(SetupPlugin)
        .add_plugin(scenes::ScenesPlugin)
        .run();
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_resource(Msaa { samples: 4 })
            .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
            .add_resource(WindowDescriptor {
                title: "AppExample".to_string(),
                width: 340,
                height: 600,
                ..Default::default()
            })
            .add_plugins(DefaultPlugins);
    }
}
