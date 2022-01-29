**References** (revised)

## Recap

We have seen how some simpler data types whose size is known at compile time are allocated on the stack.

However other types, like String, are allocated on the heap and so their memory usage needs to be managed. Rust achieves this through the principle of ownership and a set of rules that are checked at compile time. (By checking at compile time there is no extra work to do at runtime, unlike languages that use garbage collection).

Rust programmes release the heap associated with a variable when the variable’s owner goes out of scope.

Losing access to a variable when it is "moved" through re-assignment or passing to a function, is a significant constraint. Fortunately Rust gives us references which allow us to access variables without the overhead of ownership.

## References

Memory for heap-allocated data needs to be freed only once for each allocation, when the owner goes out of scope.

_References_ allow your programme to access the variable without ownership. They are not freed when the reference goes out of scope; that is still managed through the owner.

Using references we can simplify our previous example, because ownership of the word parameter doesn't move when it's accesed as a reference, and so it is still accessible after the function call.

```
fn is_palindrome(s: &String) -> bool {
    let mut normalized: String = s.chars().filter(|c| c.is_alphabetic()).collect();
    normalized = normalized.to_lowercase();
    let reversed: String = normalized.chars().rev().collect();
    reversed == normalized;
}

fn main() {
    let word1 = String::from("minim");
    let word2 = String::from("bathtub");
    let word3 = String::from("Madam I'm Adam");

    println!("{} is_palindrome? {}", word1, is_palindrome(&word1));
    println!("{} is_palindrome? {}", word2, is_palindrome(&word2));
    println!("{} is_palindrome? {}", word3, is_palindrome(&word3));
}
```

The `s` parameter passsed to `is_palindrome` is pronounced as `ref String`.

We say that `&word1` borrows a reference to `word1`. 

Exercise:

1. There is an intentional bug in the programme which prevents it from compiling successfully. Can you fix it? _Hint: what is the type of a statement?_

[details="Answer"]

The last line in `is_palindrome` has a trailing semi-colon which converts it from an expression to a statement. Remove it.

[/details]
