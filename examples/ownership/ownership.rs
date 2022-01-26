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

fn is_palindrome(s: String) -> (String, bool) {
    let mut normalized: String = s.chars().filter(|c| c.is_alphabetic()).collect();
    normalized = normalized.to_lowercase();
    println!("{}", normalized);
    let reversed: String = normalized.chars().rev().collect();
    let palindrome = reversed == normalized;
    (s, palindrome)
}

fn main() {
    let word1 = String::from("minim");
    let word2 = String::from("bathtub");
    let word3 = String::from("Madam I'm Adam");
    
    let (w1, p1) = is_palindrome(word1);
    let (w2, p2) = is_palindrome(word2);
    let (w3, p3) = is_palindrome(word3);
    println!("{} is_palindrome? {}", w1, p1);
    println!("{} is_palindrome? {}", w2, p2);
    println!("{} is_palindrome? {}", w3, p3);
}
