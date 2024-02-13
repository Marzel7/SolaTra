use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction, transaction::Transaction,
};
use std::{
    io::{self, Write},
    str::FromStr,
};

use std::{thread, time};

pub fn airdrop_sol(address: &str, sol: f64, client: &RpcClient) {
    let lamports = sol_to_lamports(sol);
    let pubkey = Pubkey::from_str(address).unwrap();
    let signature = client.request_airdrop(&pubkey, lamports).unwrap();

    let wait_milis = time::Duration::from_millis(100);
    print!("Waiting to confirm");
    io::stdout().flush().unwrap();

    loop {
        if let Ok(confirmed) = client.confirm_transaction(&signature) {
            if confirmed {
                println!("\nAirdrop to {}: {}", address, confirmed);
                break;
            }
        }
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(wait_milis);
    }
}

pub fn transfer_sol(client: &RpcClient, keypair: &Keypair, to_key: &str, sol_amount: f64) {
    let to_pubkey = match Pubkey::from_str(to_key) {
        Ok(pubkey) => pubkey,
        Err(err) => {
            println!("Error converting to Pubkey: {}", err);
            return;
        }
    };

    let lamports = sol_to_lamports(sol_amount);
    let transfer_instruction =
        system_instruction::transfer(&keypair.pubkey(), &to_pubkey, lamports);

    let latest_blockhash = client.get_latest_blockhash().unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        latest_blockhash,
    );

    let wait_milis = time::Duration::from_millis(100);
    let max_attempts = 120;
    let mut attempt_count = 0;
    println!("Waiting to confirm");
    io::stdout().flush().unwrap();

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            while attempt_count < max_attempts {
                if let Ok(confirmed) = client.confirm_transaction(&signature) {
                    if confirmed {
                        println!("\nTransfer of sol was confirmed");
                        break;
                    }
                }
                attempt_count += 1;
                println!("Attempt: {}", attempt_count);
                print!(".");
                io::stdout().flush().unwrap();
                thread::sleep(wait_milis);
            }
            println!("\nTransfer confirmation timed out");
        }
        Err(e) => {
            println!("Error transferring sol: {}", e);
        }
    }
}
