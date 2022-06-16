// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html
#![allow(dead_code, unused_variables)]

enum MyOption<T> {
    None,
    Some(T)
}

impl<T> MyOption<T> {
    fn my_unwrap(self) -> T {
        match self {
            MyOption::Some(val) => val,
            MyOption::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

fn main() {
    let with_value = MyOption::Some("there is a value");
    println!("{}", with_value.my_unwrap());

    let no_value: MyOption<String> = MyOption::None;
    // panics
    // println!("{}", no_value.my_unwrap());
}
