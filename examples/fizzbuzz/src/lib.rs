pub fn fizzbuzz(n: u64) -> String {
  "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_as_a_string() {
        assert_eq!(fizzbuzz(1), "1");
    }
}
