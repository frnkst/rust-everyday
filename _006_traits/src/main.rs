

trait Format {
    fn format_address(&self) -> String;
}

struct Address {
    street: String,
    number: i32,
    city: String
}

impl Format for Address {
    fn format_address(&self) -> String {
        let mut output = String::from("");
        output.push_str(&self.street);
        output.push_str(", ");
        output.push_str(&self.number.to_string());
        output.push_str(", ");
        output.push_str(&self.city);

        output
    }
}

fn main() {
    let address = Address {
        street: String::from("some street"),
        number: 99,
        city: String::from("some city")
    };

    println!("address is: {}", address.format_address());
}
