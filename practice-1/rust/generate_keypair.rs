use solana_sdk::signature::{Keypair, Signer};
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SerializableKeypair {
    secret: Vec<u8>,
    public: String,
}

fn main() {
    println!("🔐 Генерація нового ключа...");

    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    let secret_bytes = keypair.to_bytes().to_vec();

    let serializable = SerializableKeypair {
        secret: secret_bytes,
        public: pubkey.to_string(),
    };

    let json = serde_json::to_string_pretty(&serializable)
        .expect("❌ Помилка серіалізації ключа у JSON");

    let mut file = File::create("keypair.json")
        .expect("❌ Не вдалося створити файл keypair.json");
    file.write_all(json.as_bytes())
        .expect("❌ Не вдалося записати дані у файл");

    println!("✅ Ключ успішно збережено у keypair.json");
    println!("🧾 Публічний ключ: {}", pubkey);
}

