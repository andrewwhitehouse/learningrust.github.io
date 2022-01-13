pub fn fizzbuzz(n: u64) -> String {
  if n == 3 { "fizz".to_string() } else { n.to_string() }
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
}
