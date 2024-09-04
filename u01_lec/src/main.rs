//-------------------------- Worksheet ------------------------------------------------------------------------------------
fn fizzbuzz(n: u32) -> String {
    if n%15 == 0 {
        String::from("FizzBuzz")
    } else if n%3 == 0 {
        String::from("Fizz")
    } else if n%5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_by_3_and_5() {
       assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn test_divisible_by_3() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }

    #[test]
    fn test_divisible_by_5() {
        assert_eq!(fizzbuzz(5), "Buzz");
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(fizzbuzz(2), "2")
    }
}
