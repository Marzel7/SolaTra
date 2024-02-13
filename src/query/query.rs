use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
use std::str::FromStr;

pub fn get_supply(client: &RpcClient) {
    let supply_response = client.supply().unwrap();
    let supply = supply_response.value;

    println!(
        "Total supply: {} SOL\nCirculating: {} SOL\nNon-Circulating {} SOL",
        lamports_to_sol(supply.total),
        lamports_to_sol(supply.circulating),
        lamports_to_sol(supply.non_circulating)
    );
}

pub fn get_balance(address: &str, client: &RpcClient) {
    let pubkey = Pubkey::from_str(address).unwrap();
    let balance = client.get_balance(&pubkey).unwrap();

    println!("Balance for {}: {} ", address, lamports_to_sol(balance));
}
