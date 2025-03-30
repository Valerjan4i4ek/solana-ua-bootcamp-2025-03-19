use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
    system_instruction,
    transaction::Transaction,
};

const RPC_URL: &str = "https://api.devnet.solana.com";
const MIN_BALANCE_SOL: f64 = 0.5;
const AIRDROP_AMOUNT_SOL: f64 = 1.0;

#[derive(Deserialize)]
struct SerializableKeypair {
    secret: Vec<u8>,
    public: String,
}

fn main() {
    println!("🔍 Завантаження ключа з keypair.json...");

    let mut file = File::open("keypair.json").expect("❌ Не вдалося відкрити файл");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("❌ Не вдалося прочитати файл");

    let parsed: SerializableKeypair = serde_json::from_str(&data).expect("❌ Помилка десеріалізації");
    let keypair = Keypair::from_bytes(&parsed.secret).expect("❌ Неможливо створити Keypair");
    let pubkey = Pubkey::from_str(&parsed.public).expect("❌ Невірний формат публічного ключа");

    println!("🔑 Публічний ключ: {}", pubkey);

    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    let balance_lamports = client
        .get_balance(&pubkey)
        .expect("❌ Не вдалося отримати баланс");

    let balance_sol = balance_lamports as f64 / 1_000_000_000.0;
    println!("💰 Поточний баланс: {:.4} SOL", balance_sol);

    if balance_sol < MIN_BALANCE_SOL {
        println!("📦 Баланс менше ніж {:.2} SOL. Виконуємо airdrop на {:.2} SOL...", MIN_BALANCE_SOL, AIRDROP_AMOUNT_SOL);

        let sig = client
            .request_airdrop(&pubkey, (AIRDROP_AMOUNT_SOL * 1_000_000_000.0) as u64)
            .expect("❌ Airdrop не вдався");

        println!("⏳ Очікуємо підтвердження транзакції...");
        client
            .confirm_transaction(&sig)
            .expect("❌ Транзакцію не підтверджено");

        println!("✅ Airdrop виконано. Tx: {}", sig);

        let updated_balance = client
            .get_balance(&pubkey)
            .expect("❌ Не вдалося отримати баланс після airdrop");

        println!("💰 Новий баланс: {:.4} SOL", updated_balance as f64 / 1_000_000_000.0);
    } else {
        println!("👍 Баланс достатній, airdrop не потрібен.");
    }
}
