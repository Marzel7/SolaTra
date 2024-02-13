use chrono::{DateTime, NaiveDateTime, Utc};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::from_account, clock::Clock, commitment_config::CommitmentConfig, sysvar,
};

pub const SERVER_URL: &str = "https://api.devnet.solana.com";

pub fn get_cluster_info(client: &RpcClient) {
    let version = client.get_version().unwrap();
    let result = client
        .get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())
        .unwrap();

    let (slot, timestamp) = match result.value {
        Some(clock_account) => {
            let clock: Clock = from_account(&clock_account).unwrap();
            (result.context.slot, clock.unix_timestamp)
        }
        None => {
            panic!("Unexpected None");
        }
    };

    let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
        Utc,
    );

    println!("Cluster version: {}", version.solana_core);
    println!(
        "Block: {}, Time: {}",
        slot,
        datetime.format("%Y-%m-%d %H:%M:%S")
    );
}
