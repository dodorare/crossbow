// // use crate::button::*;
// use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
// use substrate_subxt::{ClientBuilder, KusamaRuntime};
// use tokio::sync::mpsc;

// #[cfg(not(target_os = "android"))]
// pub const TEXT_FONT_SIZE: f32 = 30.0;
// #[cfg(target_os = "android")]
// pub const TEXT_FONT_SIZE: f32 = 90.0;

// pub struct ExplorerStateChannel {
//     pub tx: mpsc::Sender<ExplorerState>,
//     pub rx: mpsc::Receiver<ExplorerState>,
// }

// impl ExplorerStateChannel {
//     pub fn new() -> Self {
//         let (tx, rx) = mpsc::channel(1);
//         Self { tx, rx }
//     }
// }

// pub fn explorer_startup(task_pool: Res<AsyncComputeTaskPool>, channel: Res<ExplorerStateChannel>) {
//     let tx = channel.tx.clone();
//     task_pool
//         .spawn(async move {
//             println!("Connecting to Substrate Node.");
//             let client = ClientBuilder::<KusamaRuntime>::new()
//                 .set_url("wss://kusama-rpc.polkadot.io")
//                 .build()
//                 .await
//                 .unwrap();
//             loop {
//                 let (best, finalized) =
//                     tokio::try_join!(client.block_hash(None), client.finalized_head()).unwrap();
//                 let (best, finalized) =
//                     tokio::try_join!(client.header(best), client.header(Some(finalized))).unwrap();
//                 let best = best.unwrap();
//                 let finalized = finalized.unwrap();
//                 tx.send(ExplorerState {
//                     best_block_number: best.number,
//                     best_block_hash: best.hash().to_string(),
//                     best_block_parent_hash: best.parent_hash.to_string(),
//                     finalized_block_number: finalized.number,
//                     finalized_block_hash: finalized.hash().to_string(),
//                     finalized_block_parent_hash: finalized.parent_hash.to_string(),
//                 })
//                 .await
//                 .unwrap();
//             }
//         })
//         .detach();
// }

// #[derive(Debug, Default, Clone)]
// pub struct ExplorerState {
//     // Best block
//     best_block_number: u32,
//     best_block_hash: String,
//     best_block_parent_hash: String,
//     // Finalized block
//     finalized_block_number: u32,
//     finalized_block_hash: String,
//     finalized_block_parent_hash: String,
// }

// #[derive(Debug, Copy, Clone)]
// pub enum Block {
//     Best(BlockTexts),
//     Finalized(BlockTexts),
// }

// #[derive(Debug, Copy, Clone)]
// pub enum BlockTexts {
//     Number,
//     Hash,
//     Parent,
// }

// pub fn explorer_text_updater(
//     mut channel: ResMut<ExplorerStateChannel>,
//     mut interaction_query: Query<(&mut Text, &Block)>,
// ) {
//     let state = channel.rx.blocking_recv().unwrap();
//     for (mut text_section, block) in interaction_query.iter_mut() {
//         let mut text = &mut text_section.sections[0];
//         match block {
//             Block::Best(texts) => match texts {
//                 BlockTexts::Number => text.value = format!("Number: {}", state.best_block_number),
//                 BlockTexts::Hash => text.value = format!("Hash: {}", state.best_block_hash),
//                 BlockTexts::Parent => {
//                     text.value = format!("Parent: {}", state.best_block_parent_hash)
//                 }
//             },
//             Block::Finalized(texts) => match texts {
//                 BlockTexts::Number => {
//                     text.value = format!("Number: {}", state.finalized_block_number)
//                 }
//                 BlockTexts::Hash => text.value = format!("Hash: {}", state.finalized_block_hash),
//                 BlockTexts::Parent => {
//                     text.value = format!("Parent: {}", state.best_block_parent_hash)
//                 }
//             },
//         };
//     }
// }

// pub fn explorer_ui(
//     commands: &mut Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
//     commands
//         .spawn(UiCameraBundle::default())
//         // root node (padding)
//         .spawn(NodeBundle {
//             style: Style {
//                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                 #[cfg(not(target_os = "ios"))]
//                 padding: Rect::all(Val::Percent(6.0)),
//                 #[cfg(target_os = "ios")]
//                 padding: Rect {
//                     top: Val::Percent(6.0),
//                     ..Default::default()
//                 },
//                 flex_direction: FlexDirection::ColumnReverse,
//                 ..Default::default()
//             },
//             material: materials.add(Color::NONE.into()),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent
//                 // explorer node
//                 .spawn(NodeBundle {
//                     style: Style {
//                         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                         flex_direction: FlexDirection::ColumnReverse,
//                         align_items: AlignItems::FlexStart,
//                         ..Default::default()
//                     },
//                     material: materials.add(Color::NONE.into()),
//                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//                     parent
//                         // best block
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 size: Size::new(Val::Percent(100.0), Val::Auto),
//                                 padding: Rect::all(Val::Percent(3.0)),
//                                 flex_direction: FlexDirection::ColumnReverse,
//                                 align_items: AlignItems::FlexStart,
//                                 ..Default::default()
//                             },
//                             material: materials.add(Color::rgba(0.15, 0.15, 0.15, 0.9).into()),
//                             ..Default::default()
//                         })
//                         .with_children(|parent| {
//                             parent
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Best block".to_string(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE / 1.5,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Number: ".to_owned(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .with(Block::Best(BlockTexts::Number))
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Hash: ".to_owned(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .with(Block::Best(BlockTexts::Hash))
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Parent: ".to_owned(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .with(Block::Best(BlockTexts::Parent));
//                         })
//                         // finalized block
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 size: Size::new(Val::Percent(100.0), Val::Auto),
//                                 margin: Rect {
//                                     top: Val::Percent(4.0),
//                                     ..Default::default()
//                                 },
//                                 padding: Rect::all(Val::Percent(3.0)),
//                                 flex_direction: FlexDirection::ColumnReverse,
//                                 align_items: AlignItems::FlexStart,
//                                 ..Default::default()
//                             },
//                             material: materials.add(Color::rgba(0.15, 0.15, 0.15, 0.9).into()),
//                             ..Default::default()
//                         })
//                         .with_children(|parent| {
//                             parent
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Finalized block".to_string(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE / 1.5,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Number: ".to_owned(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .with(Block::Finalized(BlockTexts::Number))
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Hash: ".to_owned(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .with(Block::Finalized(BlockTexts::Hash))
//                                 .spawn(TextBundle {
//                                     text: Text::with_section(
//                                         "Parent: ".to_owned(),
//                                         TextStyle {
//                                             font: font.clone(),
//                                             font_size: TEXT_FONT_SIZE,
//                                             color: Color::rgb(0.9, 0.9, 0.9),
//                                             ..Default::default()
//                                         },
//                                         Default::default(),
//                                     ),
//                                     ..Default::default()
//                                 })
//                                 .with(Block::Finalized(BlockTexts::Parent));
//                         });
//                 });
//         });
// }

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use substrate_subxt::{ClientBuilder, KusamaRuntime};
use tokio::sync::mpsc;

#[cfg(not(target_os = "android"))]
pub const TEXT_FONT_SIZE: f32 = 30.0;
#[cfg(target_os = "android")]
pub const TEXT_FONT_SIZE: f32 = 90.0;

pub struct ExplorerStateChannel {
    pub tx: mpsc::Sender<ExplorerState>,
    pub rx: mpsc::Receiver<ExplorerState>,
}

impl ExplorerStateChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self { tx, rx }
    }
}

pub fn explorer_startup(task_pool: Res<AsyncComputeTaskPool>, channel: Res<ExplorerStateChannel>) {
    let tx = channel.tx.clone();
    task_pool
        .spawn(async move {
            println!("Connecting to Substrate Node.");
            let client = ClientBuilder::<KusamaRuntime>::new()
                .set_url("wss://kusama-rpc.polkadot.io")
                .build()
                .await
                .unwrap();
            loop {
                let (best, finalized) =
                    tokio::try_join!(client.block_hash(None), client.finalized_head()).unwrap();
                let (best, finalized) =
                    tokio::try_join!(client.header(best), client.header(Some(finalized))).unwrap();
                let best = best.unwrap();
                let finalized = finalized.unwrap();
                tx.send(ExplorerState {
                    best_block_number: best.number,
                    best_block_hash: best.hash().to_string(),
                    best_block_parent_hash: best.parent_hash.to_string(),
                    finalized_block_number: finalized.number,
                    finalized_block_hash: finalized.hash().to_string(),
                    finalized_block_parent_hash: finalized.parent_hash.to_string(),
                })
                .await
                .unwrap();
            }
        })
        .detach();
}

#[derive(Debug, Default, Clone)]
pub struct ExplorerState {
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

pub fn explorer_text_updater(
    mut channel: ResMut<ExplorerStateChannel>,
    mut interaction_query: Query<(&mut Text, &Block)>,
) {
    let state = channel.rx.blocking_recv().unwrap();
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
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // button_materials: Res<ButtonMaterials>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(CameraUiBundle::default())
        // root node (padding)
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                #[cfg(not(target_os = "ios"))]
                padding: Rect::all(Val::Percent(6.0)),
                #[cfg(target_os = "ios")]
                padding: Rect {
                    top: Val::Percent(6.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // explorer node
                .spawn(NodeBundle {
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
                        .spawn(NodeBundle {
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
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Best block".to_string(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE / 1.5,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Number: ".to_owned(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Best(BlockTexts::Number))
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Hash: ".to_owned(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Best(BlockTexts::Hash))
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Parent: ".to_owned(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Best(BlockTexts::Parent));
                        })
                        // finalized block
                        .spawn(NodeBundle {
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
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Finalized block".to_string(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE / 1.5,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Number: ".to_owned(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Finalized(BlockTexts::Number))
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Hash: ".to_owned(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Finalized(BlockTexts::Hash))
                                .spawn(TextBundle {
                                    text: Text {
                                        value: "Parent: ".to_owned(),
                                        font: font.clone(),
                                        style: TextStyle {
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..Default::default()
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(Block::Finalized(BlockTexts::Parent));
                        });
                });
        });
}
