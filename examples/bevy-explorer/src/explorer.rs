use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use subxt::{OnlineClient, PolkadotConfig};
use tokio::sync::mpsc;

pub const TEXT_FONT_SIZE: f32 = 24.0;
pub const URL: &str = "wss://rpc.polkadot.io";

#[derive(Resource)]
pub struct ExplorerStateChannel {
    tx: mpsc::Sender<ExplorerState>,
    rx: mpsc::Receiver<ExplorerState>,
}

impl ExplorerStateChannel {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self { tx, rx }
    }
}

pub fn explorer_startup(channel: Res<ExplorerStateChannel>) {
    let tx = channel.tx.clone();
    AsyncComputeTaskPool::get()
        .spawn(async move {
            let runtime = tokio::runtime::Runtime::new().expect("create Tokio runtime");
            runtime.block_on(async move {
                if let Err(error) = stream_blocks(tx).await {
                    error!("Polkadot block stream stopped: {error:#}");
                }
            });
        })
        .detach();
}

async fn stream_blocks(tx: mpsc::Sender<ExplorerState>) -> anyhow::Result<()> {
    info!("Connecting to {URL}");
    let api = OnlineClient::<PolkadotConfig>::from_url(URL).await?;
    let mut best_blocks = api.stream_best_blocks().await?;
    let mut finalized_blocks = api.stream_blocks().await?;
    let mut state = ExplorerState::default();

    loop {
        tokio::select! {
            block = best_blocks.next() => {
                let block = block.ok_or_else(|| anyhow::anyhow!("best-block stream ended"))??;
                state.best = BlockState {
                    number: block.number(),
                    hash: block.hash().to_string(),
                    parent_hash: block.header().parent_hash.to_string(),
                };
            }
            block = finalized_blocks.next() => {
                let block = block.ok_or_else(|| anyhow::anyhow!("finalized-block stream ended"))??;
                state.finalized = BlockState {
                    number: block.number(),
                    hash: block.hash().to_string(),
                    parent_hash: block.header().parent_hash.to_string(),
                };
            }
        }

        // The UI only needs the most recent snapshot; skipping a full buffer is intentional.
        let _ = tx.try_send(state.clone());
    }
}

#[derive(Debug, Default, Clone)]
struct ExplorerState {
    best: BlockState,
    finalized: BlockState,
}

#[derive(Debug, Default, Clone)]
struct BlockState {
    number: u64,
    hash: String,
    parent_hash: String,
}

#[derive(Debug, Copy, Clone, Component)]
pub enum BlockText {
    BestNumber,
    BestHash,
    BestParent,
    FinalizedNumber,
    FinalizedHash,
    FinalizedParent,
}

pub fn explorer_text_updater(
    mut channel: ResMut<ExplorerStateChannel>,
    mut texts: Query<(&mut Text, &BlockText)>,
) {
    let Ok(state) = channel.rx.try_recv() else {
        return;
    };

    for (mut text, field) in &mut texts {
        text.0 = match field {
            BlockText::BestNumber => format!("Number: {}", state.best.number),
            BlockText::BestHash => format!("Hash: {}", state.best.hash),
            BlockText::BestParent => format!("Parent: {}", state.best.parent_hash),
            BlockText::FinalizedNumber => format!("Number: {}", state.finalized.number),
            BlockText::FinalizedHash => format!("Hash: {}", state.finalized.hash),
            BlockText::FinalizedParent => format!("Parent: {}", state.finalized.parent_hash),
        };
    }
}

pub fn explorer_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            padding: UiRect::all(percent(6)),
            flex_direction: FlexDirection::Column,
            row_gap: px(24),
            ..default()
        })
        .with_children(|root| {
            spawn_block_panel(
                root,
                "Best block",
                [
                    BlockText::BestNumber,
                    BlockText::BestHash,
                    BlockText::BestParent,
                ],
            );
            spawn_block_panel(
                root,
                "Finalized block",
                [
                    BlockText::FinalizedNumber,
                    BlockText::FinalizedHash,
                    BlockText::FinalizedParent,
                ],
            );
        });
}

fn spawn_block_panel(parent: &mut ChildSpawnerCommands, title: &str, fields: [BlockText; 3]) {
    parent
        .spawn((
            Node {
                width: percent(100),
                padding: UiRect::all(px(20)),
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.9)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new(title),
                TextFont::from_font_size(TEXT_FONT_SIZE * 1.25),
                TextColor(Color::WHITE),
            ));
            for (label, field) in ["Number: ", "Hash: ", "Parent: "].into_iter().zip(fields) {
                panel.spawn((
                    Text::new(label),
                    TextFont::from_font_size(TEXT_FONT_SIZE),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    field,
                ));
            }
        });
}
