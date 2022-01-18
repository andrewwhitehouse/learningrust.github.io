/*
fn gcd(a: u64, b: u64) -> u64 {
    // Shadow parameters with mutable variables
    let mut b = b;
    let mut a = a;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
  println!("{}", gcd(10, 2));
}
*/


/*
fn gcd(a: u64, b: u64) -> u64 {
    // Shadow parameters with mutable variables
    let mut b = b;
    let mut a = a;
    loop {
        let temp = b;
        b = a % b;
        a = temp;
        if b == 0 {
            break a;
        }
    }
}

fn main() {
  println!("{}", gcd(10, 2));
}
*/

/*
fn main() {
    let mut sum = 0;
    let values = [1,3,5,8,17];
    for v in values {
        sum += v;    
    }
    println!("{}", sum);
}
*/


/*
fn main() {
    let values = [1,3,5,8,17];
    let mut sum = 0;
    for index in 0..values.len() {
        sum += values[index];
    }
    println!("{}", sum);
}
*/

fn simplify((numerator, denominator): (u64, u64)) -> (u64, u64) {
    let mut b = numerator;
    let mut a = denominator;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    let gcd = a;
    (numerator / gcd, denominator / gcd)  
}

fn main() {
  println!("{:?}", simplify((357, 1411)));
}
