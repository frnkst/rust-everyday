fn main() {
    // everything normal here
    say_hello("frank");

    // pass argument seperately
    say_some_more("you are nice!", say_hello, "other person");

    // pass a closure
    say_some_more("you are nice!", |i| println!("something else {}", i), "bla");

    // use closure to bind a parameter to a function and don't execute it yet
    let say_hello_to_franky = move || say_hello("franky");
    execute_callback_function(say_hello_to_franky);

    let say_you = move |x, y| say_three_things("you", x, y);
    let say_you_are = move |y| say_you("are", y);
    say_you_are("getting the hang of the closures");

    fn say_you_without_closure(x: &str, y: &str) {
        say_three_things("you", x, y);
    }

    let say_you_are2 = move |y| say_you("are", y);
    say_you_are2("works too");
    say_you_without_closure("a", "b")
}

fn say_hello(name: &str) {
    println!("hello {}", name);
}

fn say_three_things(one: &str, two: &str, three: &str) {
    println!("{} {} {}", one, two, three);
}

fn say_some_more(more: &str, hello: fn(&str), arg: &str) {
    hello(arg);
    println!("and one more thing: {}", more);
}

fn execute_callback_function(callback: fn()) {
    callback();
}
