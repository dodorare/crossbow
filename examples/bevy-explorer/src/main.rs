mod explorer;

use bevy::{prelude::*, window::WindowResolution};
use explorer::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Explorer".to_owned(),
                resolution: WindowResolution::new(640, 1200),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ExplorerStateChannel::new())
        .add_systems(Startup, (explorer_startup, explorer_ui))
        .add_systems(Update, explorer_text_updater)
        .run();
}
