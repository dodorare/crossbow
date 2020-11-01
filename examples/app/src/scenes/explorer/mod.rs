use bevy::prelude::*;

pub struct ExplorerScene;

impl Plugin for ExplorerScene {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.add_startup_system(substrate.system());
    }
}

fn substrate(task_pool: Res<bevy::tasks::IoTaskPool>) {
    task_pool
        .spawn(async {
            println!("Connecting to Substrate Node.");
            let client = substrate_subxt::ClientBuilder::<substrate_subxt::KusamaRuntime>::new()
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
        .detach();
}
