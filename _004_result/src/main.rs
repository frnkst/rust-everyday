#![allow(dead_code)]

fn main() {
    let result = square_value(500);
    safe_print_using_match(result);

    let result_too_big = square_value(5000000);
    safe_print_using_match(result_too_big);

    let result = square_value(500);
    safe_print_using_if_let(result);

    let result_too_big = square_value(5000000);
    safe_print_using_if_let(result_too_big);
}


#[derive(Debug, PartialEq)] // Use derives
enum SquareValueError { // Define a custom error type
OverflowError,
    SomeOtherError
}

fn square_value(i: i32) -> Result<i32, SquareValueError> {
    match i <= i32::MAX / i {
        true => Ok(i * i),
        false => Err(SquareValueError::OverflowError),
    }
}

fn safe_print_using_match(result: Result<i32, SquareValueError>) {
    match result {
        Ok(value) => println!("Got a result {}", value),
        Err(SquareValueError::OverflowError) => println!("got an overflow error"),
        _ => println!("something else went wrong")
    }
}

fn safe_print_using_if_let(result: Result<i32, SquareValueError>) {
    if let Ok(value) = result {
        println!("also have a result some other name: {}", value);
    } else {
        println!("also have an error but I can't access the type");
    }

    if let Err(SquareValueError::OverflowError) = result {
        println!("here I can access the type of error");
    }
}

