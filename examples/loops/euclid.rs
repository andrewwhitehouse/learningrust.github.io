fn gcd(a: u64, b: u64) -> u64 {
    let mut m_b = b;
    let mut m_a = a;
    while m_b != 0 {
        let temp = m_b;
        m_b = m_a % m_b;
        m_a = temp;
    }
    m_a
}

fn main() {
  println!("{}", gcd(10, 2));
}
