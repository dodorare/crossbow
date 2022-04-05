use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra};
use tokio::sync::mpsc;

#[subxt::subxt(runtime_metadata_path = "res/metadata.scale")]
pub mod bevy_explorer {}

#[cfg(not(target_os = "android"))]
pub const TEXT_FONT_SIZE: f32 = 30.0;
#[cfg(target_os = "android")]
pub const TEXT_FONT_SIZE: f32 = 30.0;

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
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                println!("Connecting to Substrate Node");
                let api = ClientBuilder::new()
                    .set_url("wss://rpc.polkadot.io:443")
                    .build()
                    .await
                    .unwrap()
                    .to_runtime_api::<bevy_explorer::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();
                let client = api.client.rpc();
                loop {
                    let (block_hash, finalized_head) = tokio::try_join!(
                        client.block_hash(None),
                        client.finalized_head()
                    )
                    .unwrap();
                    let (best, finalized) = tokio::try_join!(
                        client.header(block_hash),
                        client.header(Some(finalized_head))
                    )
                    .unwrap();
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
            });
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

#[derive(Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum Block {
    Best(BlockTexts),
    Finalized(BlockTexts),
}

impl Default for Block {
    fn default() -> Self {
        Self::Best(BlockTexts::Number)
    }
}

#[derive(Debug, Copy, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum BlockTexts {
    Number,
    Hash,
    Parent,
}

impl Default for BlockTexts {
    fn default() -> Self {
        Self::Number
    }
}

pub fn explorer_text_updater(
    mut channel: ResMut<ExplorerStateChannel>,
    mut interaction_query: Query<(&mut Text, &Block), (With<Block>,)>,
) {
    let state = channel.rx.blocking_recv().unwrap();
    for (mut text, block) in interaction_query.iter_mut() {
        match block {
            Block::Best(texts) => match texts {
                BlockTexts::Number => {
                    text.sections[0].value = format!("Number: {}", state.best_block_number)
                }
                BlockTexts::Hash => {
                    text.sections[0].value = format!("Hash: {}", state.best_block_hash)
                }
                BlockTexts::Parent => {
                    text.sections[0].value = format!("Parent: {}", state.best_block_parent_hash)
                }
            },
            Block::Finalized(texts) => match texts {
                BlockTexts::Number => {
                    text.sections[0].value = format!("Number: {}", state.finalized_block_number)
                }
                BlockTexts::Hash => {
                    text.sections[0].value = format!("Hash: {}", state.finalized_block_hash)
                }
                BlockTexts::Parent => {
                    text.sections[0].value = format!("Parent: {}", state.best_block_parent_hash)
                }
            },
        };
    }
}

pub fn explorer_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    // Root node (padding)
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                #[cfg(not(target_os = "ios"))]
                padding: Rect {
                    left: Val::Percent(6.0),
                    right: Val::Percent(6.0),
                    top: Val::Percent(6.0),
                    bottom: Val::Percent(18.0),
                },
                #[cfg(target_os = "ios")]
                padding: Rect {
                    top: Val::Percent(6.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Explorer node
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Best block
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: Rect::all(Val::Percent(3.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::rgba(0.15, 0.15, 0.15, 0.9).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Best block",
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: TEXT_FONT_SIZE / 1.5,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Number: ",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Block::Best(BlockTexts::Number));
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Hash: ",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Block::Best(BlockTexts::Hash));
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Parent: ",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Block::Best(BlockTexts::Parent));
                        });

                    // Finalized block
                    parent
                        .spawn_bundle(NodeBundle {
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
                            color: Color::rgba(0.15, 0.15, 0.15, 0.9).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Finalized block",
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: TEXT_FONT_SIZE / 1.5,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Number: ",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Block::Finalized(BlockTexts::Number));
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Hash: ",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Block::Finalized(BlockTexts::Hash));
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Parent: ",
                                        TextStyle {
                                            font: font_handle.clone(),
                                            font_size: TEXT_FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Block::Finalized(BlockTexts::Parent));
                        });
                });
        });
}
