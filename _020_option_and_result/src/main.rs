use rand::Rng;

fn main() {
    do_the_work().expect("Expected a value");
}

fn do_the_work() -> Result<i32, ()> {
    let maybe_value = maybe_get_value()?;
    Ok(maybe_value)
}

fn maybe_get_value() -> Result<i32, ()> {
    let mut rng = rand::thread_rng();
    let random_value= rng.gen_range(0..1);
    if random_value == 5 {
        return Ok(1);
    }
    return Err(())
}
