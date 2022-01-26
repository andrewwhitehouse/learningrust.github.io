**Ownership and Functions**

Passing a variable to a function will move or copy, just as assignment does. 

```
fn is_palindrome(s: String) -> bool {
    let reversed: String = s.chars().rev().collect();
    reversed == s
}

fn main() {
    let word1 = String::from("minim");
    let word2 = String::from("bathtub");
    let word3 = String::from("Madam I'm Adam");   
    println!("is_palindrome? {}", is_palindrome(word1));
    println!("is_palindrome? {}", is_palindrome(word2));
    println!("is_palindrome? {}", is_palindrome(word3));
}
```

If you try to print the value of the word parameter in either of the print statements you will see a compiler error similar to this:

```
move out of `word1` occurs here
```

Try it.

```
    println!("{} is_palindrome? {}", word1, is_palindrome(word1));
    println!("{} is_palindrome? {}", word2, is_palindrome(word2));
    println!("{} is_palindrome? {}", word3, is_palindrome(word3));
```

To work around this issue we can return the argument passed to the function, like this:

```
    let (w1, p1) = is_palindrome(word1);
    let (w2, p2) = is_palindrome(word2);
    let (w3, p3) = is_palindrome(word3);
    println!("{} is_palindrome? {}", w1, p1);
    println!("{} is_palindrome? {}", w2, p2);
    println!("{} is_palindrome? {}", w3, p3);
```    

In the next section we'll learn about references, which will help us to avoid the complication of returning parameters as above.

Exercises:

1. Modify is_palindrome so that it returns a the tuple of type (String, bool) representing the word, and whether it is a palindrome.

2. The third word _is_ a palindrome if you consider only the alphabetic characters and ignore the case. Fix the function so that "Madam I'm Adam" is treated as a palindrome. 

[details="Solutions"]

### 1
```
fn is_palindrome(s: String) -> (String, bool) {
    let reversed: String = s.chars().rev().collect();
    let palindrome = reversed == s;
    (s, palindrome)
}
```

### 2

```
fn is_palindrome(s: String) -> (String, bool) {
    let mut normalized: String = s.chars().filter(|c| c.is_alphabetic()).collect();
    normalized = normalized.to_lowercase();
    println!("{}", normalized);
    let reversed: String = normalized.chars().rev().collect();
    let palindrome = reversed == normalized;
    (s, palindrome)
}
```

[/details]
