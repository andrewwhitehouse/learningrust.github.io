fn add(a: i32, maybe_b: Option<i32>, maybe_c: Option<i32>) -> i32{
  let b = if maybe_b.is_none() { 0 } else { maybe_b.unwrap() };
  let c = if maybe_c.is_none() { 0 } else { maybe_c.unwrap() };
  a + b + c
} 

fn main() {
  println!("{}", add(3, None, None));
  println!("{}", add(3, Some(8), None));
  println!("{}", add(3, Some(8), Some(6)));
}
