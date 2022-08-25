use bevy::prelude::*;

fn main() {
    println!("Initialization.");
    std::thread::sleep(std::time::Duration::from_secs(2));
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_startup_system(audio)
        .add_startup_system(icon)
        .run();
}

fn icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/icon.png"),
        ..Default::default()
    });
}

// fn audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     let music = asset_server.load("sounds/Windless-Slopes.mp3");
//     audio.play(music);
// }
