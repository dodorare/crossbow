use super::button::*;

use bevy::prelude::*;

pub struct ExplorerScene;

impl Plugin for ExplorerScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_resource::<ButtonMaterials>()
            .add_startup_system(explorer_background.system())
            .add_startup_system(explorer_ui.system())
            .add_system(button_effect.system())
            .add_system(moving_cube.system());
    }
}

#[cfg(not(target_os = "android"))]
pub const TEXT_FONT_SIZE: f32 = 30.0;
#[cfg(target_os = "android")]
pub const TEXT_FONT_SIZE: f32 = 90.0;

// fn substrate(task_pool: Res<bevy::tasks::IoTaskPool>) {
//     task_pool
//         .spawn(async {
//             println!("Connecting to Substrate Node.");
//             let client = substrate_subxt::ClientBuilder::<substrate_subxt::KusamaRuntime>::new()
//                 .set_url("wss://kusama-rpc.polkadot.io")
//                 .build()
//                 .await
//                 .unwrap();
//             let block_number = 1;
//             let block_hash = client.block_hash(Some(block_number.into())).await.unwrap();
//             if let Some(hash) = block_hash {
//                 println!("Block hash for block number {}: {}", block_number, hash);
//             } else {
//                 println!("Block number {} not found.", block_number);
//             }
//         })
//         .detach();
// }

fn explorer_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(UiCameraComponents::default())
        // root node (padding)
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: Rect::all(Val::Percent(6.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // explorer node
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        // explorer info block
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: Rect::all(Val::Percent(3.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    text: Text {
                                        value: "Height: 234242".to_string(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .spawn(TextComponents {
                                    text: Text {
                                        value: "best/finalize".to_string(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .spawn(TextComponents {
                                    text: Text {
                                        value: "Recent block: 1db...1fe".to_string(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                });
                        });
                });

            parent
                // explorer buttons node
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(12.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        // button
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Undefined,
                                    bottom: Val::Undefined,
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: "Refresh".to_string(),
                                    font,
                                    style: TextStyle {
                                        font_size: BUTTON_FONT_SIZE,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .with(Interaction::default());
                });
        });
}

struct MovingCube;

fn moving_cube(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut Handle<StandardMaterial>, &MovingCube)>,
) {
    for (mut transform, mut material, _) in query.iter_mut() {
        if transform.translation.x() < 1.0 && transform.translation.z() < 2.0 {
            *transform.translation.x_mut() += time.delta_seconds * 0.8;
        } else if transform.translation.z() < 2.0 && transform.translation.x() >= 1.0 {
            *transform.translation.z_mut() += time.delta_seconds * 0.8;
        } else if transform.translation.x() > -1.0 && transform.translation.z() >= 2.0 {
            *transform.translation.x_mut() -= time.delta_seconds * 0.8;
        } else if transform.translation.z() >= 0.0 {
            *transform.translation.z_mut() -= time.delta_seconds * 0.8;
        }
        // println!("Hi");
    }
}

fn explorer_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: standard_materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: standard_materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(MovingCube)
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
