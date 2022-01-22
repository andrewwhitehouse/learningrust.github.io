/*
fn main() {
   println!("Before {}", s); // 1
   if true {
     println!("Before declaration {}", s); // 2
     let s = "Hello World";
     println!("s: {}", s); // 3
   }
   println!("After {}", s); // 4
}
*/

fn main() {
   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{}", s);
}
