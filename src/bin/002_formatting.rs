fn main() {
    println!("hello rust-everyday");
    println!("hello {} using placeholders", "rust-everyday");
    println!("hello {1} using {0}", "placeholders", "rust-everyday");
    println!("hello {name} using named {ph}", ph = "placeholders", name = "rust-everyday");
    println!("debug output: {:?}", (true, 77, "test"));
    println!("number formatting as decimal hex {:x} and octal {:o}", 10, 10);
}
