fn main() {
        println!("Everyday rust!");
    }

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add2() {
        assert_eq!(add(1, 2), 3);
    }
}

