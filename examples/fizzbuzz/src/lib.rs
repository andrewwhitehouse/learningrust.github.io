pub fn fizzbuzz(n: u64) -> String {
    let is_multiple_of_3 = n % 3 == 0;
    if is_multiple_of_3 {
        "fizz".to_string()
    } else if n % 5 == 0 {
        "buzz".to_string()
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_one_as_a_string() {
        assert_eq!(fizzbuzz(1), "1");
    }

    #[test]
    fn it_returns_two_as_a_string() {
        assert_eq!(fizzbuzz(2), "2");
    }

    #[test]
    fn it_returns_fizz_for_3() {
        assert_eq!(fizzbuzz(3), "fizz");
    }

    #[test]
    fn it_returns_buzz_for_5() {
        assert_eq!(fizzbuzz(5), "buzz");
    }

    #[test]
    fn it_returns_fizz_for_6() {
        assert_eq!(fizzbuzz(6), "fizz");
    }

    #[test]
    fn it_returns_buzz_for_10() {
        assert_eq!(fizzbuzz(10), "buzz");
    }

    #[test]
    fn it_returns_fizzbuzz_for_15() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
    }
}
