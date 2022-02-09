fn add(i: &i32, j: &i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add(&a, &b);
    println!("{}", res);
}
