// mod button;
mod explorer;

// use button::*;
use explorer::*;

use bevy::prelude::*;

#[creator::creator_main]
pub fn main() {
    println!("Initialization.");
    App::build()
        .add_resource(ClearColor(Color::rgb(0.18, 0.17, 0.16)))
        .add_resource(WindowDescriptor {
            title: "Explorer".to_string(),
            width: 640,
            height: 1200,
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
