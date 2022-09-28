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
    #[cfg(not(target_os = "android"))]
    let image_path = get_assets_path();
    #[cfg(target_os = "android")]
    let image_path = std::path::PathBuf::from("images").join("icon.png");
    let asset: Handle<Image> = asset_server.load(image_path);
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset,
        ..Default::default()
    });
}

// fn audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     let music = asset_server.load("sounds/Windless-Slopes.mp3");
//     audio.play(music);
// }

/// Workaround. Failed to get assets on windows from the .load() method through the
/// relative path to asset
fn get_assets_path() -> std::path::PathBuf {
    let font_path = std::path::PathBuf::from("assets")
        .join("images")
        .join("icon.png");
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_path = manifest_dir.parent().unwrap().parent().unwrap();
    let image_path = assets_path.join(font_path);
    image_path
}
