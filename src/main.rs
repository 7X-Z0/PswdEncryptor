use std::io::{self, Write};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, Nonce, OsRng},
    ChaCha20Poly1305,
};
use rand::Rng;

fn main() {
    let mut input = String::new();
    let key: [u8; 32] = rand::rng().gen();

    print!("Enter something: ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
}