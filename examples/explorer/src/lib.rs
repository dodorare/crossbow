mod button;
mod explorer;

// use button::*;
use bevy::prelude::*;
use explorer::*;
use std::sync::{Arc, RwLock};

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
        .init_resource::<RwLock<ExplorerState>>()
        .init_resource::<RwLock<LocalClient>>()
        // .init_resource::<ButtonMaterials>()
        // .add_system(button_effect)
        // .add_system(explorer_button)
        .add_startup_system(explorer_ui)
        .add_system(substrate)
        .add_system(explorer_text_updater)
        .run();
}
