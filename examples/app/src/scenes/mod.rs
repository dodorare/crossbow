#[cfg(feature = "explorer")]
mod explorer;
#[cfg(feature = "menu")]
mod menu;
#[cfg(feature = "paint")]
mod paint;

use bevy::prelude::*;

pub struct ScenesPlugin;
impl Plugin for ScenesPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        #[cfg(feature = "explorer")]
        app_builder.add_plugin(explorer::ExplorerScene);

        #[cfg(feature = "paint")]
        app_builder.add_plugin(paint::PaintScene);

        #[cfg(feature = "menu")]
        app_builder.add_plugin(menu::MenuScene);

        // TODO: Move them to own scenes.
        // app_builder
        //     .add_startup_system(audio.system())
        //     .add_startup_system(helmet.system())
        //     .add_startup_system(icon.system());
    }
}

fn icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("branding/icon.png");
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

fn helmet(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_scene(asset_server.load("models/helmet/FlightHelmet.gltf"))
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.7, 0.7, 1.0))
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::unit_y()),
            ..Default::default()
        });
}

fn audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/Windless-Slopes.mp3");
    audio.play(music);
}
