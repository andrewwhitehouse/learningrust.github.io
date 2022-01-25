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

/*
// This won't compile
fn main() {
   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{}", s);
   let s2 = s;
   println!("{}", s);
}
*/

/*
fn main() {
   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{}", s);
   let s2 = s.clone();
   println!("{} {}", s, s2);
}
*/

fn is_palindrome(s: String) -> bool {
    let reversed: String = s.chars().rev().collect();
    reversed == s
}

fn main() {
    let word1 = String::from("minim");
    let word2 = String::from("Madam I'm Adam");
    println!("{} is_palindrom? {}", word1, is_palindrome(word1));
    println!("{} is_palindrom? {}", word2, is_palindrome(word2));
}
