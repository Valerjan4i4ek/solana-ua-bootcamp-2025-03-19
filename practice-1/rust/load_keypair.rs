use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use solana_sdk::signature::{Keypair, Signer};

#[derive(Deserialize)]
struct SerializableKeypair {
    secret: Vec<u8>,
    public: String,
}

fn main() {
    println!("🔑 Завантаження ключа з файлу...");

    let mut file = File::open("keypair.json").expect("❌ Не вдалося відкрити keypair.json");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("❌ Не вдалося прочитати дані з файлу");

    let deserialized: SerializableKeypair =
        serde_json::from_str(&data).expect("❌ Помилка десеріалізації ключа");

    let keypair = Keypair::from_bytes(&deserialized.secret)
        .expect("❌ Не вдалося відновити ключ із secret");

    println!("✅ Ключ завантажено!");
    println!("Публічний ключ: {}", keypair.pubkey());
}
