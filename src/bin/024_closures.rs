fn main() {
    let this_is_a_closure = || println!("in the closure");
    this_is_a_closure();

    let closure_with_a_value = |x| println!("in the closure with a value {}", x);
    closure_with_a_value("the value");

    // move a value into the closure
    let a = vec![1, 2, 3];
    let moved_closure = move || println!("{a:?}");
    moved_closure();

    // a can't be used anymore because it has been moved into the closure
    // println!("a is {a:?}");

    let a = vec![1, 2, 3];
    let mapped_vec: Vec<i32> = a.iter().map(|x| x + 1).collect();
    println!("{mapped_vec:?}");

    let b = vec![4,5,6];
    let add_one = |x| x + 1;
    let m: Vec<i32> = b.iter().map(add_one).collect();
    println!("{m:?}");
}
