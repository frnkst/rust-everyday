use std::io::Write;
use rpassword::read_password;
use sha1::{Sha1, Digest};

fn main() {
    let password = get_password();
    let full_hash = hash_password(&password);
    let start_hash = &full_hash[..5];

    let url = "https://api.pwnedpasswords.com/range/".to_owned() + start_hash;
    let body = reqwest::blocking::get(&url).unwrap().text().unwrap();

    for line in body.lines() {
        let result: Vec<&str> = line.split(":").collect();
        let end_of_hash = result[0];
        let number_of_breaches = result[1];
        if start_hash.to_owned() + end_of_hash == full_hash.to_owned() {
            println!("Password found in {} breaches", number_of_breaches);
        }
    }
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();

    hex::encode_upper(result).to_owned()
}

fn get_password() -> String {
    println!("Password to be checked: ");
    std::io::stdout().flush().unwrap();

    read_password().unwrap()
}
