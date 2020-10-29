#![allow(dead_code)]

mod ui;

use bevy::prelude::*;
use substrate_subxt::{ClientBuilder, KusamaRuntime};
use ui::{button_system, ui_setup, ButtonMaterials};

#[cfg(target_os = "android")]
use android_logger::Config;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_min_level(log::Level::Trace));

    println!("Initialization.");
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
        .add_resource(WindowDescriptor {
            title: "AppExample".to_string(),
            width: 400,
            height: 660,
            ..Default::default()
        })
        .add_default_plugins()
        .init_resource::<ButtonMaterials>()
        .add_startup_system(ui_setup.system())
        .add_system(button_system.system())
        .add_startup_system(audio.system())
        .add_startup_system(substrate.system())
        // .add_startup_system(helmet.system())
        // .add_startup_system(icon.system())
        .run();
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

fn substrate() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Connecting to Substrate Node.");
            let client = ClientBuilder::<KusamaRuntime>::new()
                .set_url("wss://kusama-rpc.polkadot.io")
                .build()
                .await
                .unwrap();
            let block_number = 1;
            let block_hash = client.block_hash(Some(block_number.into())).await.unwrap();
            if let Some(hash) = block_hash {
                println!("Block hash for block number {}: {}", block_number, hash);
            } else {
                println!("Block number {} not found.", block_number);
            }
        })
}
