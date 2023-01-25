fn main() {
    let x = 500;
    println!("number: {}", x);
    // x = 501; can't assign to immutable variable

    let mut y = 500;
    println!("mutable number: {}", y);
    y = 501; // works because the variable is mutable
    println!("mutable number: {}", y);

    let z = 500; // warning unused mut
    // z = "test"; mismatched types, expected i32 but got &str
    println!("number: {}", z);

    const ID: i32 = 001; // const always needs to have a type
    println!("const: {}", ID);

    let a_char = 'a';
    println!("char: {}", a_char);

    let is_active = true;
    println!("bool: {}", is_active);

    let a_string = "rust-everyday";
    println!("string: {}", a_string);

    let an_array = ["a", "b", "c"]; // array with list notation
    an_array.map(|it| { println!("array: {}", it); });

    let other_array: [&str; 3] = ["a"; 3]; // array with repeat expression [type, length] = [value, repetition]
    other_array.map(|it| { println!("other array: {}", it); });

    //for loop through array
    for _x in other_array {
        println!("{_x}");
    }

    // but this works also
    for _x in &other_array {
        println!("{_x}");
    }

    // a list is called a vector
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // loop through a vector
    for _x in &vec {
        println!("{_x}");
    }

    assert_eq!(vec.len(), 2);
    assert_eq!(vec.pop(), Some(2));

    // initialize the vector like this
    let _vec1 = vec![1, 2, 3];

    // or also like this
    let _vec2 = Vec::from([1, 2, 3, 4]);
}
