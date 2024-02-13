use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::read_keypair_file, signer::Signer};

mod query;
mod rpc;
mod transaction;
mod wallet;

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
    KeyGen {
        #[arg(short, long, help = "Output filepath for keypair file")]
        output: String,
        #[arg(
            short,
            long,
            default_value_t = 12,
            help = "How many words to generate for the mnemonic. Valid values are: 12, 15, 18, 21, and 24."
        )]
        mnemonic_word_count: u32,
        #[arg(short, long, help = "Passphrase to use for extra security.")]
        passphrase: Option<String>,
    },
    Balance {
        #[arg(group = "input")]
        address: Option<String>,
        #[arg(long, group = "input")]
        wallet_file: Option<String>,
    },
    Airdrop {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        sol: f64,
    },
    Transfer {
        #[arg(short, long)]
        from_wallet: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        sol: f64,
    },
}

fn main() {
    let cli = Cli::parse();
    let client = RpcClient::new(rpc::rpc::SERVER_URL);

    match &cli.command {
        Some(Commands::ClusterInfo) => {
            println!("Get cluster info");
            rpc::rpc::get_cluster_info(&client);
        }
        Some(Commands::Supply) => {
            println!("Get supply info");
            query::query::get_supply(&client)
        }
        Some(Commands::KeyGen {
            output,
            mnemonic_word_count,
            passphrase,
        }) => {
            print!("Generate keys, output to: {}", output);
            wallet::wallet::generate_keypair(output, *mnemonic_word_count as usize, passphrase);
        }
        Some(Commands::Balance {
            address,
            wallet_file,
        }) => {
            if let Some(address) = address {
                println!("Get balance for address: {}", address);
                query::query::get_balance(address, &client)
            } else if let Some(wallet_path) = wallet_file {
                println!("Get balance for wallet file: {} ", wallet_path);
                let keypair = read_keypair_file(wallet_path).unwrap();
                query::query::get_balance(&keypair.pubkey().to_string(), &client);
            }
        }
        Some(Commands::Airdrop { address, sol }) => {
            println!("Airdrop {} SOl to {:?}", sol, address);
            transaction::transaction::airdrop_sol(address, *sol, &client);
        }
        Some(Commands::Transfer {
            from_wallet,
            to,
            sol,
        }) => {
            let keypair = read_keypair_file(from_wallet).unwrap();
            println!("Transfer {} SOL from {} to {}", sol, &keypair.pubkey(), to);

            query::query::get_balance(&keypair.pubkey().to_string(), &client);

            transaction::transaction::transfer_sol(&client, &keypair, to, *sol);
        }

        None => {}
    }
}

// cargo run --release -- key-gen --output ./keypair.json --mnemonic-word-count 12

// cargo run --release -- balance --wallet-file "./keypair.json"
// ./target/release/bot1 balance "Dt2vw1Y7UDvXpBKapUC59CHv2K3AG6Azv5anjTMgAcHx"

// cargo run --release -- airdrop -a FhQB9aRjnyvWg96yPua8a8cp9smiras2f4JaMuf2dJjF -s 2

// generate new wallet
// ./target/release/bot1 key-gen -o ./wallet2.json -m 12

// transfer from one address to another
// ./target/release/bot1 transfer --from-wallet ./keypair.json --to Dt2vw1Y7UDvXpBKapUC59CHv2K3AG6Azv5anjTMgAcHx --sol 0.05
