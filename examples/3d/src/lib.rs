// use bevy::prelude::*;

// #[creator::creator_main]
// pub fn main() {
//     println!("Initialization.");
//     App::build()
//         .insert_resource(Msaa { samples: 4 })
//         .insert_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(helmet.system())
//         .run();
// }

// fn helmet(commands: &mut Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn_scene(asset_server.load("models/helmet/FlightHelmet.gltf#Scene0"))
//         .spawn(LightBundle {
//             transform: Transform::from_xyz(4.0, 5.0, 4.0),
//             ..Default::default()
//         })
//         .spawn(PerspectiveCameraBundle {
//             transform: Transform::from_xyz(0.7, 0.7, 1.0)
//                 .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::unit_y()),
//             ..Default::default()
//         });
// }
