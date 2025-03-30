import { Keypair } from "@solana/web3.js";

const prefix = "VAL";

let attempts = 0;
while (true) {
  const keypair = Keypair.generate();
  const publicKey = keypair.publicKey.toBase58();
  attempts++;

  if (publicKey.startsWith(prefix)) {
    console.log("🎉 Знайдено ключ!");
    console.log("Спроб:", attempts);
    console.log("🔑 Public Key:", publicKey);
    console.log("🔐 Secret Key:", `[${keypair.secretKey.toString()}]`);
    break;
  }

  if (attempts % 1000 === 0) {
    console.log("Спроб:", attempts, "Ідемо далі...");
  }
}
