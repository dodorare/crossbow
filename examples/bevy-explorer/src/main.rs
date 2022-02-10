#![allow(dead_code)]

mod explorer;

use bevy::{core::FixedTimestep, prelude::*};
use explorer::*;

fn main() {
    println!("Initialization.");
    std::thread::sleep(std::time::Duration::from_secs(2));
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WindowDescriptor {
            title: "Explorer".to_string(),
            width: 640.0,
            height: 1200.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ExplorerStateChannel::new())
        .add_startup_system(explorer_startup.system())
        .add_startup_system(explorer_ui.system())
        .add_stage_after(
            CoreStage::Update,
            "my_stage",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(explorer_text_updater.system()),
        )
        .run();
}
