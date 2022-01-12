fn fib_recur(n: usize) -> u64 {
    if n < 2 {
       return n as u64
    }
    fib_recur(n-1) + fib_recur(n-2)
}

fn fib_iter(n: usize) -> u64 {
    if n == 0 {
       return 0
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _index in 1..n {
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    b
}

fn main() {
  for n in 0..=25 {
    println!("{} {}", n, fib_iter(n))
  }
}
