use aes::{Aes128, Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::{fs, str};
use std::borrow::Borrow;
use std::env;
use std::fs::File;
use std::io::Write;
use owo_colors::OwoColorize;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let iv = get_random_hex();
    let key = get_random_hex();

    let file_path =  "/Users/frank/Code/rust-everyday/bla.txt";
    let original_file = fs::read(file_path).unwrap();

    println!("Message: {}",message);
    println!("Key: {}", hex::encode(&key));
    println!("IV: {}", hex::encode(&iv));

    let ciphertext = encrypt(&iv, &key, &original_file);

    let encrypted_file = file_path.to_owned() + ".enc";
    let mut a = File::create(encrypted_file).expect("ha");
    a.write_all(&ciphertext).unwrap(); // writing using the macro 'writeln!'

    println!("Encrypted message: {:?}", hex::encode(&ciphertext));

    let enc_file = fs::read("/Users/frank/Code/rust-everyday/bla.txt.enc").unwrap();

    let decrypted_ciphertext = decrypt(&iv, &key, &enc_file);
    println!("Decrypted message: {:?}",str::from_utf8(&*decrypted_ciphertext).unwrap());
}

fn encrypt(iv: &Vec<u8>, key: &Vec<u8>, file: &Vec<u8>) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let file_length: usize = file.len();
    let padding: usize = file_length % 128;

    let mut buffer = vec![0u8; file_length + padding];
    buffer[..file_length].copy_from_slice(&file);

    cipher.encrypt(&mut buffer, file_length).unwrap().to_vec()
}

fn decrypt(iv: &Vec<u8>, key: &Vec<u8>, ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buf = ciphertext.to_vec();

    cipher.decrypt(&mut buf).unwrap().to_owned()
}

fn get_random_hex() -> Vec<u8> {
    (0..16)
        .map(|_| {
            thread_rng().gen_range(0..16)
        }).collect::<Vec<u8>>()
}
