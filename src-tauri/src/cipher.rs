use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme, PublicKey};
use rand::rngs::OsRng;

fn main() {
    // Генерация ключей
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Сообщение для шифрования
    let message = "хуем по лбу".as_bytes();
    
    // Шифрование
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key.encrypt(&mut rng, padding, &message[..]).expect("Failed to encrypt");

    println!("Encrypted message: {:?}", encrypted_data);

    // Расшифровка
    let padding = PaddingScheme::new_pkcs1v15_encrypt(); // Используем ту же схему
    let decrypted_data = private_key.decrypt(padding, &encrypted_data).expect("Failed to decrypt");

    println!("Decrypted message: {:?}", String::from_utf8(decrypted_data).expect("Failed to convert to string"));
}
