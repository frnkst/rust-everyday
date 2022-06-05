

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
        let s = &self.street;
        let n = &self.number;
        let c = &self.city;

        s.to_owned() + "," + &*n.to_string().to_owned() + "," + c
    }
}

fn main() {
    let address = Address {
        street: String::from("some street"),
        number: 99,
        city: String::from("some city")
    };

    println!("address is: {}", address.concatenate());
}
