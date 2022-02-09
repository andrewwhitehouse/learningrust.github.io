**Lifetimes**

I have been trying to make sense of Rust _lifetimes_. I've previously written about how variable memory usage is controlled by their owner (of which there can be only one) and how _references_ can be used to access variables without the overhead of ownership.

**But** those references cannot outlive the variable that they refer to (or "borrow" from).

I tried the example in Rust in Action (2.8 Advanced Function Definitions) which uses lifetime annotations ... I'll get to what these look like in a minute.

Here is the function without the annotations:

```
fn add(i: &i32, j: &i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add(&a, &b);
    println!("{}", res);
}
```

And it compiles. 

Here is the example from the book:

```
fn add<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add(&a, &b);
    println!("{}", res);
}
```

The revised function defines two lifetime variables _a_ and _b_, and binds these to the function parameters.

Every parameter has a lifetime but these can usually be inferred by the compiler.

Anyway, in this example the lifetime annotations don't appear to be adding anything.

Here is a better, adapted, [example](https://stackoverflow.com/questions/31609137/why-are-explicit-lifetimes-needed-in-rust/31609892#31609892) from Stack Overflow:

```
fn foo<'a, 'b>(x: &'a u32, _y: &'b u32) -> &'a u32 {
    x
}

fn main() {
    let x = 12;
    let z: &u32 = {
        let y = 42;
        foo(&x, &y)
    };
    println!("{}", z);
}
```

This prints 12.

If we change the function a little:

```
fn foo<'a, 'b>(_x: &'a u32, y: &'b u32) -> &'b u32 {
    y
}

fn main() {
    let x = 12;
    let z: &u32 = {
        let y = 42;
        foo(&x, &y)
    };
    println!("{}", z);
}
```

Now the compiler complains:

```
error[E0597]: `y` does not live long enough
  --> main.rs:9:17
   |
7  |     let z: &u32 = {
   |         - borrow later stored here
8  |         let y = 42;
9  |         foo(&x, &y)
   |                 ^^ borrowed value does not live long enough
10 |     };
   |     - `y` dropped here while still borrowed

error: aborting due to previous error
```

So we're printing the value of z, which is calculated using the borrowed value of y, which goes out of scope on line 10 at the closing brace.

This does make me think: why bother with the block to calculate z? 

Why not simply do this?

```
fn foo<'a, 'b>(_x: &'a u32, y: &'b u32) -> &'b u32 {
    y
}

fn main() {
    let x = 12;
    let y = 42;
    let z: &u32 = foo(&x, &y);
    println!("{}", z);
}
```

I guess if there were something computationally expensive being created in that block and we wanted to free it as soon as possible then it might make sense. But are we really going to be agonising over those kind of details. I hope this is an edge case because I find the syntax rather gnarly.
