fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-5, -7), -12);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_should_overflow_max() {
        assert_eq!(add(i32::MAX, 1), 0);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_should_overflow_min() {
        assert_eq!(add(i32::MIN, -1), 0);
    }
}

fn main() {
    let _a = add(5,2);
}
