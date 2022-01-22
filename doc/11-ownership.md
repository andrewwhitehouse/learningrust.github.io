**Memory Management in Rust: Ownership**

Rust takes an approach to memory management that avoids the need to explicitly allocate and free memory, or the overhead of garbage collection.

Each value in Rust has a variable that's called its owner.

There can be only one owner at a time.

When the owner goes out of scope, that value will be _dropped_, meaning that Rust will call its `drop()` function to free any resources.

[_The first two of the last 3 sentences were takes from the [Rust docs](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) -- check whether I need to reference the original material, and how._]

A variable comes into scope when it is declared, and goes out of scope when execution passes the closing brace in which it is declared. [_Are there any exceptions to this?_]

For example:

```
fn main() {
   println!("Before {}", s); // 1
   if true {
     println!("Before declaration {}", s); // 2
     let s = "Hello World";
     println!("s: {}", s); // 3
   }
   println!("After {}", s); // 4
}
```

To make this programme compile you would need to comment out print statements at commented points 1, 2 and 4. Because the Rust compiler complains that it can't find the variable in that scope.

Types whose size is known at compile time are stored on the heap, including string literals. However, data types whose size isn't known at compile time are stored on the heap. One example of this is the String type.

```
let mut s = String::from("foo");
s.push_str("bar");
println!("{}", s);
```

Strings can be mutated and so are managed on the heap.

Rust tracks the ownership of these heap variables and frees up the allocated space (via `drop()`) when the variable goes out of scope.

If we make a small change to the above code

```
let mut s = String::from("foo");
s.push_str("bar");
println!("{}", s);
let s2 = s;
println!("{}", s);
``` 

In this case, ownership of the String has transferred to `s2` with the assignment, and so this code will not compile. Rust considers `s` as no longer valid after `s` is ***moved*** into `s2` (and won't try to free the storage associated with `s` when it goes out of scope).

If we do want to make a copy of `s` we can use `clone()`

```
   let mut s = String::from("foo");
   s.push_str("bar");
   println!("{}", s);
   let s2 = s.clone();
   println!("{} {}", s, s2);
```

which creates a _deep_ copy of the data, with no underlying references to the original.

## The Copy trait

Types whose size is known at compile time, and are stored on the stack, can be copied cheaply. Types that implement the Copy trait support a deep copy, and their variables are still valid after assignment to another variable.

As a rule of thumb, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. 

Here are some of the types that implement Copy:

* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

[_Copyright now - most of the copy trait text was copied from [the docs](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)_]

## Ownership and Functions

[_hook for next post ..._]


