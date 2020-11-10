// use crate::button::*;
use bevy::prelude::*;
use substrate_subxt::{Client, ClientBuilder, KusamaRuntime};

#[cfg(not(target_os = "android"))]
pub const TEXT_FONT_SIZE: f32 = 30.0;
#[cfg(target_os = "android")]
pub const TEXT_FONT_SIZE: f32 = 90.0;

pub fn substrate(
    task_pool: Res<bevy::tasks::IoTaskPool>,
    mut local_client: ResMut<LocalClient>,
    mut state: ResMut<State>,
) {
    local_client.counter += 1;
    if local_client.counter % 10 == 0 {
        task_pool.scope(|s| {
            s.spawn(async move {
                if local_client.client.is_none() {
                    println!("Connecting to Substrate Node.");
                    local_client.client = Some(
                        ClientBuilder::<KusamaRuntime>::new()
                            .set_url("wss://rpc.polkadot.io")
                            .build()
                            .await
                            .unwrap(),
                    );
                }
                let client = local_client.client.clone().unwrap();
                let res = futures::try_join!(client.block_hash(None), client.finalized_head());
                let (best, finalized) = res.unwrap();
                let res = futures::try_join!(client.header(best), client.header(Some(finalized)));
                if let Ok((Some(best), Some(finalized))) = res {
                    state.best_block_number = best.number;
                    state.best_block_hash = best.hash().to_string();
                    state.best_block_parent_hash = best.parent_hash.to_string();
                    state.finalized_block_number = finalized.number;
                    state.finalized_block_hash = finalized.hash().to_string();
                    state.finalized_block_parent_hash = finalized.parent_hash.to_string();
                }
            })
        });
    }
}

#[derive(Clone)]
pub struct LocalClient {
    client: Option<Client<KusamaRuntime>>,
    counter: i32,
}

impl Default for LocalClient {
    fn default() -> Self {
        Self {
            client: None,
            counter: -1,
        }
    }
}

#[derive(Default, Clone)]
pub struct State {
    // Best block
    best_block_number: u32,
    best_block_hash: String,
    best_block_parent_hash: String,
    // Finalized block
    finalized_block_number: u32,
    finalized_block_hash: String,
    finalized_block_parent_hash: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Block {
    Best(BlockTexts),
    Finalized(BlockTexts),
}

#[derive(Debug, Copy, Clone)]
pub enum BlockTexts {
    Number,
    Hash,
    Parent,
}

pub fn explorer_text_updater(state: Res<State>, mut interaction_query: Query<(&mut Text, &Block)>) {
    for (mut text, block) in interaction_query.iter_mut() {
        match block {
            Block::Best(texts) => match texts {
                BlockTexts::Number => text.value = format!("Number: {}", state.best_block_number),
                BlockTexts::Hash => text.value = format!("Hash: {}", state.best_block_hash),
                BlockTexts::Parent => {
                    text.value = format!("Parent: {}", state.best_block_parent_hash)
                }
            },
            Block::Finalized(texts) => match texts {
                BlockTexts::Number => {
                    text.value = format!("Number: {}", state.finalized_block_number)
                }
                BlockTexts::Hash => text.value = format!("Hash: {}", state.finalized_block_hash),
                BlockTexts::Parent => {
                    text.value = format!("Parent: {}", state.best_block_parent_hash)
                }
            },
        };
    }
}

// pub struct ExplorerButton;

pub fn explorer_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // button_materials: Res<ButtonMaterials>,
    state: Res<State>,
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
                                            font_size: TEXT_FONT_SIZE / 1.5,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!("Number: {}", state.best_block_number),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Best(BlockTexts::Number))
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!("Hash: {}", state.best_block_hash),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Best(BlockTexts::Hash))
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!("Parent: {}", state.best_block_parent_hash),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Best(BlockTexts::Parent));
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
                                            font_size: TEXT_FONT_SIZE / 1.5,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!("Number: {}", state.finalized_block_number),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Finalized(BlockTexts::Number))
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!("Hash: {}", state.finalized_block_hash),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Finalized(BlockTexts::Hash))
                                .spawn(TextComponents {
                                    text: Text {
                                        value: format!(
                                            "Parent: {}",
                                            state.finalized_block_parent_hash
                                        ),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Finalized(BlockTexts::Parent));
                        });
                });

            // parent
            //     // explorer buttons node
            //     .spawn(NodeComponents {
            //         style: Style {
            //             size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
            //             flex_direction: FlexDirection::ColumnReverse,
            //             ..Default::default()
            //         },
            //         material: materials.add(Color::NONE.into()),
            //         ..Default::default()
            //     })
            //     .with_children(|parent| {
            //         parent
            //             // run audio button
            //             .spawn(NodeComponents {
            //                 style: Style {
            //                     size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            //                     margin: Rect {
            //                         top: Val::Percent(4.0),
            //                         bottom: Val::Percent(6.0),
            //                         ..Default::default()
            //                     },
            //                     justify_content: JustifyContent::Center,
            //                     align_items: AlignItems::Center,
            //                     ..Default::default()
            //                 },
            //                 material: button_materials.normal.clone(),
            //                 ..Default::default()
            //             })
            //             .with(ExplorerButton)
            //             .with(Interaction::default())
            //             .with_children(|parent| {
            //                 parent.spawn(TextComponents {
            //                     text: Text {
            //                         value: "Refresh".to_string(),
            //                         font: font.clone(),
            //                         style: TextStyle {
            //                             font_size: BUTTON_FONT_SIZE,
            //                             color: Color::rgb(0.9, 0.9, 0.9),
            //                         },
            //                     },
            //                     ..Default::default()
            //                 });
            //             });
            //     });
        });
}

// pub fn explorer_button(
//     task_pool: Res<bevy::tasks::IoTaskPool>,
//     local_client: Res<LocalClient>,
//     mut state: ResMut<State>,
//     interaction_query: Query<(&Node, Mutated<Interaction>, &ExplorerButton)>,
// ) {
//     for (_, interaction, _) in interaction_query.iter() {
//         let client = local_client.client.clone().unwrap();
//         let state = &mut state;
//         match *interaction {
//             Interaction::Clicked => {
//                 task_pool.scope(|s| {
//                     s.spawn(async move {
//                         let res =
//                             futures::try_join!(client.block_hash(None), client.finalized_head());
//                         let (best, finalized) = res.unwrap();
//                         let res =
//                             futures::try_join!(client.header(best), client.header(Some(finalized)));
//                         if let Ok((Some(best), Some(finalized))) = res {
//                             state.best_block_number = best.number;
//                             state.best_block_hash = best.hash().to_string();
//                             state.best_block_parent_hash = best.parent_hash.to_string();
//                             state.finalized_block_number = finalized.number;
//                             state.finalized_block_hash = finalized.hash().to_string();
//                             state.finalized_block_parent_hash = finalized.parent_hash.to_string();
//                         }
//                     })
//                 });
//             }
//             Interaction::Hovered => (),
//             Interaction::None => (),
//         }
//     }
// }
