mod button;
mod explorer;

// use button::*;
use bevy::{core::FixedTimestep, prelude::*};
use explorer::*;

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
        .add_resource(ExplorerStateChannel::new())
        // .init_resource::<ButtonMaterials>()
        .add_startup_system(explorer_startup)
        .add_startup_system(explorer_ui)
        .add_stage_after(
            stage::UPDATE,
            "substrate_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(explorer_text_updater),
        )
        // .add_system(button_effect)
        // .add_system(explorer_button)
        // .add_startup_system(explorer_ui)
        // .add_system(explorer_text_updater)
        .run();
}
