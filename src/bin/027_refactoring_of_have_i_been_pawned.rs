use std::io::Write;
use rpassword::read_password;
use sha1::{Sha1, Digest};

fn main() {
    let password = get_password();
    let (hashed_password, start_hashed_password) = hash_password(&password);
    let response = get_matching_hashes(&start_hashed_password);
    if let Ok(response) = response {
        let number_of_breaches = find_exact_hash(hashed_password, start_hashed_password, response);
        print_result(&number_of_breaches)
    }
}

fn get_password() -> String {
    println!("Password to be checked: ");
    std::io::stdout().flush().unwrap();
    read_password().unwrap()
}

fn hash_password(password: &str) -> (String, String) {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let full_password_hash = hex::encode_upper(result).to_owned();
    let start_hashed_password = full_password_hash[..5].to_owned();
    (full_password_hash, start_hashed_password)
}

fn get_matching_hashes(start_hashed_password: &str) -> Result<String, reqwest::Error> {
    let url = "https://api.pwnedpasswords.com/range/".to_owned() + start_hashed_password;
    let response = reqwest::blocking::get(&url)?;
    let response_body = response.text()?;
    Ok(response_body)
}


fn find_exact_hash(hashed_password: String, start_hashed_password: String, response: String) -> i32 {
    for line in response.lines() {
        let (end_hashed_password, number_of_breaches) = parse_line(line);
        if start_hashed_password.to_owned() + end_hashed_password == hashed_password.to_owned() {
            return number_of_breaches;
        }
    }
    0
}

fn parse_line(line: &str) -> (&str, i32) {
    let one_match: Vec<&str> = line.split(":").collect();
    let end_hashed_password = one_match[0];
    let number_of_breaches = one_match[1].parse::<i32>().unwrap();
    (end_hashed_password, number_of_breaches)
}

fn print_result(number_of_breaches: &i32) {
    if number_of_breaches > &0 {
        println!("Password found in {} breaches", number_of_breaches);
    } else {
        println!("Password not found in any breaches");
    }
}
