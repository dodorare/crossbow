use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};

fn main() {
    println!("Initialization.");
    std::thread::sleep(std::time::Duration::from_secs(2));
    App::new()
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: "wgpu=debug,wgpu_hal=debug,bevy_render=info".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(audio)
        .add_startup_system(icon)
        .run();
}

fn icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        ..Default::default()
    });
}

fn audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/Windless-Slopes.mp3");
    audio.play(music);
}
