fn is_palindrome(s: &String) -> bool {
    let mut normalized: String = s.chars().filter(|c| c.is_alphabetic()).collect().to_lowercase();
    let reversed: String = normalized.chars().rev().collect();
    reversed == normalized
}

fn main() {
    let word1 = String::from("minim");
    let word2 = String::from("bathtub");
    let word3 = String::from("Madam I'm Adam");
    
    println!("{} is_palindrome? {}", word1, is_palindrome(&word1));
    println!("{} is_palindrome? {}", word2, is_palindrome(&word2));
    println!("{} is_palindrome? {}", word3, is_palindrome(&word3));
}
