**Ownership and Functions**

Passing a variable to a function will move or copy, just as assignment does. 

```
fn takes_ownership_of(s: String) {
    println!("In takes_ownership_of s={}", s);
}

fn main() {
    let s = String::from("Giraffe");
    takes_ownership_of(s);
    println!("After function call s={}", s);
}
```

The above code won't compile, as it attempts to access `s` after it has been passed to the function.

If you want to access the value passed to the function after it is called, you could return the parameter (transferring ownership back to the caller).

```
fn takes_ownership_of(s: String) {
    println!("In takes_ownership_of s={}", s);
    s
}
```

But there is a more straightforward way ...

