use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::from_account, clock::Clock, commitment_config::CommitmentConfig,
    native_token::lamports_to_sol, sysvar,
};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ClusterInfo,
    Supply,
}

const SERVER_URL: &str = "https://api.devnet.solana.com";

fn get_cluster_info(client: &RpcClient) {
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

fn get_supply(client: &RpcClient) {
    let supply_response = client.supply().unwrap();
    let supply = supply_response.value;

    println!(
        "Total supply: {} SOL\nCirculating: {} SOL\nNon-Circulating {} SOL",
        lamports_to_sol(supply.total),
        lamports_to_sol(supply.circulating),
        lamports_to_sol(supply.non_circulating)
    );
}

fn main() {
    let cli = Cli::parse();
    let client = RpcClient::new(SERVER_URL);

    match &cli.command {
        Some(Commands::ClusterInfo) => {
            println!("Get cluster info");
            get_cluster_info(&client)
        }
        Some(Commands::Supply) => {
            println!("Get supply info");
            get_supply(&client)
        }
        None => {}
    }
}

// clap - attibute providing metadata such as author, version and description
// struct Cli -  single field named command, an Option wrapping an enum 'Commands'

// Commands enum represents different subcommands the application can accept.
// In this case  a single variant named 'ClusterInfo'

// main - parses the command-line arguments into an instance of the 'Cli' struct using Cli::parse()
// then uses matchto check which statement was providede
// If the subcommand is ClusterInfo. it prints 'get Cluster Info'
// if no subcommand is provided, it prints out a message