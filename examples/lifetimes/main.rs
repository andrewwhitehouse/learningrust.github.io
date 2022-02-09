fn foo<'a, 'b>(_x: &'a u32, y: &'b u32) -> &'b u32 {
    y
}

fn main() {
    let x = 12;
    let y = 42;
    let z: &u32 = foo(&x, &y);
    println!("{}", z);
}
