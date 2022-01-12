fn fib(n: usize) -> u64 {
    if n == 0 {
       return 0
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _index in 2..n {
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    b
}

fn main() {
  println!("{}", fib(25))
}
