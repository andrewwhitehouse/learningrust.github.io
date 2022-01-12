fn overeningeered_add(a: u32, b: u32) -> u32 {
  if b == 0 { a } else { add(a+1, b-1) }
} 

fn main() {
  println!("{}", overengineered_add(123, 456));
}
