mod line;
mod paint;

use bevy::prelude::*;

#[creator::creator_main]
pub fn main() {
    println!("Initialization.");
    App::build()
        .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(paint::paint_setup.system())
        .add_system_to_stage(stage::FIRST, paint::paint_system.system())
        .run();
}
