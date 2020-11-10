// mod button;
mod explorer;

// use button::*;
use explorer::*;

use bevy::prelude::*;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    println!("Initialization.");
    App::build()
        .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
        .add_resource(WindowDescriptor {
            title: "Explorer".to_string(),
            width: 340,
            height: 600,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .init_resource::<LocalClient>()
        // .init_resource::<ButtonMaterials>()
        // .add_system(button_effect.system())
        // .add_system(explorer_button.system())
        .add_startup_system(explorer_ui.system())
        .add_system(substrate.system())
        .add_system(explorer_text_updater.system())
        .run();
}
