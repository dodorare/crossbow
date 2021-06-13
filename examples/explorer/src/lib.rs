mod explorer;

use bevy::{core::FixedTimestep, prelude::*};
use explorer::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct SubstrateUpdate;

#[creator::creator_main]
pub fn main() {
    println!("Initialization.");
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WindowDescriptor {
            title: "Explorer".to_string(),
            width: 640.0,
            height: 1200.0,
            ..Default::default()
        })
        .insert_resource(ExplorerStateChannel::new())
        .add_plugins(DefaultPlugins)
        .add_startup_system(explorer_startup.system())
        .add_startup_system(explorer_ui.system())
        .add_stage_after(
            CoreStage::Update,
            SubstrateUpdate,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(explorer_text_updater.system()),
        )
        .run();
}
