fn main() {
 say_hello("frank");

 // pass argument seperately
 say_some_more("you are nice!", say_hello, "other person");

 // pass a closure
 say_some_more("you are nice!", |i| println!("something else {}", i), "bla");
}

fn say_hello(name: &str) {
    println!("hello {}", name);
}

fn say_some_more(more: &str, hello: fn(&str), arg: &str) {
    hello(arg);
    println!("and one more thing: {}", more);
}
