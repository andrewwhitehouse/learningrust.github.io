fn fibonacci() -> [u64; 90] {
    let mut fib = [0; 90];
    fib[0] = 0;
    fib[1] = 1;
    for index in 2..fib.len() {
        fib[index] = fib[index-1] + fib[index-2];
    }
    return fib;
}

fn main() {
    let result = fibonacci();
    println!("{}", result[75] + result[68]);

    //let ratio = result[result.len()-1] as f64 / result[result.len()-2] as f64;
    //println!("{}", ratio);
}
