#![allow(dead_code)]

#[cfg(target_os = "android")]
use android_logger::Config;

use bevy::prelude::*;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "full"))]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_min_level(log::Level::Trace));

    println!("The world!");

    // #[cfg(target_os = "android")]
    // enumerate_audio_devices().unwrap();

    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_startup_system(cube.system())
        .add_startup_system(text.system())
        // .add_startup_system(audio.system())
        // .add_startup_system(monkey.system())
        // .add_startup_system(icon.system())
        .run();
}

/// set up a simple 3D scene
fn cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(-3.0, 3.0, 5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("branding/icon.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

fn text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf").unwrap();
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::Center,
                margin: Rect::all(Val::Px(100.0)),
                ..Default::default()
            },
            text: Text {
                value: "It works".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 280.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        });
}

fn monkey(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // mesh
        .spawn(PbrComponents {
            // load a mesh from glTF
            mesh: asset_server
                .load("models/monkey/Monkey.gltf")
                .unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(-1.5, 0.0, 0.0)),
            ..Default::default()
        })
        // mesh
        .spawn(PbrComponents {
            // load a mesh from binary glTF
            mesh: asset_server
                .load("models/monkey/Monkey.glb")
                .unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(1.5, 0.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(-2.0, 2.0, 6.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn audio(asset_server: Res<AssetServer>, audio_output: Res<AudioOutput>) {
    let music = asset_server.load("sounds/Windless Slopes.mp3").unwrap();
    audio_output.play(music);
}

#[cfg(target_os = "android")]
fn enumerate_audio_devices() -> Result<(), Box<dyn std::error::Error>> {
    const GET_DEVICES_OUTPUTS: jni::sys::jint = 2;

    // Create a VM for executing Java calls
    let native_activity = ndk_glue::native_activity();
    let vm_ptr = native_activity.vm();
    let vm = unsafe { jni::JavaVM::from_raw(vm_ptr) }?;
    let env = vm.attach_current_thread()?;

    // Query the global Audio Service
    let class_ctxt = env.find_class("android/content/Context")?;
    let audio_service = env.get_static_field(class_ctxt, "AUDIO_SERVICE", "Ljava/lang/String;")?;

    let audio_manager = env
        .call_method(
            native_activity.activity(),
            "getSystemService",
            // JNI type signature needs to be derived from the Java API
            // (ArgTys)ResultTy
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[audio_service],
        )?
        .l()?;

    // Enumerate output devices
    let devices = env.call_method(
        audio_manager,
        "getDevices",
        "(I)[Landroid/media/AudioDeviceInfo;",
        &[GET_DEVICES_OUTPUTS.into()],
    )?;

    println!("-- Output Audio Devices --");

    let device_array = devices.l()?.into_inner();
    let len = env.get_array_length(device_array)?;
    for i in 0..len {
        let device = env.get_object_array_element(device_array, i)?;

        // Collect device information
        // See https://developer.android.com/reference/android/media/AudioDeviceInfo
        let product_name: String = {
            let name =
                env.call_method(device, "getProductName", "()Ljava/lang/CharSequence;", &[])?;
            let name = env.call_method(name.l()?, "toString", "()Ljava/lang/String;", &[])?;
            env.get_string(name.l()?.into())?.into()
        };
        let id = env.call_method(device, "getId", "()I", &[])?.i()?;
        let ty = env.call_method(device, "getType", "()I", &[])?.i()?;

        let sample_rates = {
            let sample_array = env
                .call_method(device, "getSampleRates", "()[I", &[])?
                .l()?
                .into_inner();
            let len = env.get_array_length(sample_array)?;

            let mut sample_rates = vec![0; len as usize];
            env.get_int_array_region(sample_array, 0, &mut sample_rates)?;
            sample_rates
        };

        println!("Device {}: Id {}, Type {}", product_name, id, ty);
        println!("sample rates: {:#?}", sample_rates);
    }

    Ok(())
}
