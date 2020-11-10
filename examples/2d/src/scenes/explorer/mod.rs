use super::button::*;

use bevy::prelude::*;

pub struct ExplorerScene;

impl Plugin for ExplorerScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .init_resource::<ButtonMaterials>()
            .add_resource(MovingCubeState { is_moving: true })
            .add_startup_system(explorer_background.system())
            .add_startup_system(explorer_ui.system())
            .add_system(button_effect.system())
            .add_system(explorer_button.system())
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

enum ExplorerButton {
    MovingCube,
    RunAudio,
}

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
                        // best block
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: Rect::all(Val::Percent(3.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgba(0.15, 0.15, 0.15, 0.9).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    text: Text {
                                        value: "Best block".to_string(),
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
                                        value: "Number: 1234212".to_string(),
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
                                        value: "Hash: 0x314...122".to_string(),
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
                                        value: "Parent: 0x314...121".to_string(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                });
                        })
                        // finalized block
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                margin: Rect {
                                    top: Val::Percent(4.0),
                                    ..Default::default()
                                },
                                padding: Rect::all(Val::Percent(3.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgba(0.15, 0.15, 0.15, 0.9).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    text: Text {
                                        value: "Finalized block".to_string(),
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
                                        value: "Number: 2234212".to_string(),
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
                                        value: "Hash: 0x114...122".to_string(),
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
                                        value: "Parent: 0x114...121".to_string(),
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
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        // stop/move cube button
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .with(ExplorerButton::MovingCube)
                        .with(Interaction::default())
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: "Stop/Move Cube".to_string(),
                                    font: font.clone(),
                                    style: TextStyle {
                                        font_size: BUTTON_FONT_SIZE,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        // run audio button
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                margin: Rect {
                                    top: Val::Percent(4.0),
                                    bottom: Val::Percent(6.0),
                                    ..Default::default()
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .with(ExplorerButton::RunAudio)
                        .with(Interaction::default())
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: "Run Audio".to_string(),
                                    font: font.clone(),
                                    style: TextStyle {
                                        font_size: BUTTON_FONT_SIZE,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                },
                                ..Default::default()
                            });
                        });
                });
        });
}

fn explorer_button(
    mut moving_cube: ResMut<MovingCubeState>,
    interaction_query: Query<(&Node, Mutated<Interaction>, &ExplorerButton)>,
) {
    for (_node, interaction, button) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button {
                ExplorerButton::MovingCube => {
                    moving_cube.is_moving = !moving_cube.is_moving;
                }
                ExplorerButton::RunAudio => {}
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

struct MovingCube;

struct MovingCubeState {
    is_moving: bool,
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

// should be (1.0 % CUBE_SPEED == 0)
const CUBE_SPEED: f32 = 0.05;

fn moving_cube(moving_cube: Res<MovingCubeState>, mut query: Query<(&mut Transform, &MovingCube)>) {
    for (mut transform, _) in query.iter_mut() {
        if moving_cube.is_moving {
            if transform.translation.x() < 1.0 && transform.translation.z() == 0.0 {
                let x = transform.translation.x();
                transform
                    .translation
                    .set_x((x * 100.0).round() / 100.0 + CUBE_SPEED);
            } else if transform.translation.z() < 2.0 && transform.translation.x() == 1.0 {
                let z = transform.translation.z();
                transform
                    .translation
                    .set_z((z * 100.0).round() / 100.0 + CUBE_SPEED);
            } else if transform.translation.x() > -1.0 && transform.translation.z() == 2.0 {
                let x = transform.translation.x();
                transform
                    .translation
                    .set_x((x * 100.0).round() / 100.0 - CUBE_SPEED);
            } else if transform.translation.z() > 0.0 {
                let z = transform.translation.z();
                transform
                    .translation
                    .set_z((z * 100.0).round() / 100.0 - CUBE_SPEED);
            }
        }
    }
}
