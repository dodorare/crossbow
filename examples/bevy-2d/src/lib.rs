use bevy::prelude::*;

// Bevy supplies the native Android entry point. Crossbow can therefore build this package through
// Cargo's public CLI without rewriting rustc invocations or generating engine-specific glue.
#[bevy_main]
pub fn main() {
    println!("Initialization.");
    std::thread::sleep(std::time::Duration::from_secs(2));
    let mut app = App::new();

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: desktop_assets_path().to_string_lossy().into_owned(),
        ..default()
    }));
    #[cfg(any(target_os = "android", target_os = "ios"))]
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, icon).run();
}

fn icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asset: Handle<Image> = asset_server.load("images/icon.png");
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset));
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn desktop_assets_path() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets")
}
