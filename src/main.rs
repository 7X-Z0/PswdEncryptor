use std::io::{self, Write};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use base64::{engine::general_purpose, Engine};

fn main() {
    let mut input = String::new();
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    print!("Enter something: ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let ciphertext_result = cipher.encrypt(&nonce, input.as_bytes());
    match ciphertext_result {
        Ok(ciphertext) => {
            let base64_ciphertext = general_purpose::STANDARD.encode(&ciphertext);
            println!("Ciphertext: {}", base64_ciphertext);
        }
        Err(e) => {
            println!("Encryption failed: {:?}", e);
        }
    }

    
}