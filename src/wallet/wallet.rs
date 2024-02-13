use bip39::{Language, Mnemonic, MnemonicType, Seed};

use solana_sdk::{
    signature::{keypair_from_seed, write_keypair_file},
    signer::Signer,
};

pub fn generate_keypair(
    output_path: &str,
    mnemonic_word_count: usize,
    passphrase: &Option<String>,
) {
    let mnemonic_type = MnemonicType::for_word_count(mnemonic_word_count).unwrap();
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);

    let seed = match passphrase {
        Some(phrase) => Seed::new(&mnemonic, phrase),
        None => Seed::new(&mnemonic, ""),
    };

    let keypair = keypair_from_seed(seed.as_bytes()).unwrap();
    write_keypair_file(&keypair, output_path).unwrap();

    println!("Mnemonic: {:?}", mnemonic);
    println!("Public key: {}", &keypair.pubkey());
}
