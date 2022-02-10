use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u128 = 10000000000;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("a is less than b");
    }
}
