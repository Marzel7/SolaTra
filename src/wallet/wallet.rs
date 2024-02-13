use bip39::{Language, Mnemonic, MnemonicType, Seed};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::keypair_from_seed, signer::Signer};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone)]
struct KeyPairInfo {
    address: String,
    keypair: Vec<u8>,
    seed_phrase: String,
}

pub fn generate_keypair(
    output_path: &str,
    mnemonic_word_count: usize,
    passphrase: &Option<String>,
    count: usize,
    new_wallet: &bool,
) {
    if *new_wallet {
        delete_file(output_path)
    };

    for _ in 0..count {
        let mnemonic_type = MnemonicType::for_word_count(mnemonic_word_count).unwrap();
        let mnemonic: Mnemonic = Mnemonic::new(mnemonic_type, Language::English);

        let seed = match passphrase {
            Some(phrase) => Seed::new(&mnemonic, phrase),
            None => Seed::new(&mnemonic, ""),
        };

        let keypair = keypair_from_seed(seed.as_bytes()).unwrap();

        println!("Mnemonic: {:?}", mnemonic);
        println!("Public key: {}", &keypair.pubkey());

        let keypair_info = KeyPairInfo {
            keypair: keypair.to_bytes().to_vec(),
            address: keypair.pubkey().to_string(),
            seed_phrase: mnemonic.phrase().to_string(),
        };

        write_keypair_file(&keypair_info, output_path);
    }
}

fn write_keypair_file(keypair_info: &KeyPairInfo, output_path: &str) {
    let formatted_keypair_info = format!(
        "{{\n  \"address\": \"{}\",\n  \"keypair\": [{}],\n  \"seed_phrase\": \"{}\"\n}}",
        keypair_info.address,
        keypair_info
            .keypair
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        keypair_info.seed_phrase
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true) // Open the file in append mode
        .open(output_path)
        .expect("Failed to open file");

    writeln!(file, "{}", formatted_keypair_info).expect("Failed to write to file");
}

fn delete_file(file_path: &str) {
    if let Err(err) = fs::remove_file(file_path) {
        eprintln!("Error deleting file: {}", err);
    } else {
        println!("File deleted successfully: {}", file_path);
    }
}
