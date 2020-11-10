use crate::button::*;
use bevy::prelude::*;

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

pub enum ExplorerButton {
    RunAudio,
}

pub fn explorer_ui(
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
                                    value: "Refresh".to_string(),
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

pub fn explorer_button(interaction_query: Query<(&Node, Mutated<Interaction>, &ExplorerButton)>) {
    for (_node, interaction, button) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button {
                ExplorerButton::RunAudio => {}
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
