**More Mutable References**

I could claim -- in order to justify my writing this book -- that Rust is the most amazing language ever and it's all going to be upside. You may find, as I have, that some of the design decisions in Rust are close enough to other languages, but with important differences. And so that requires work to integrate these into the mental models you've developed so far.

Language designers trade off developer productivity against hardware cycles. Rust makes you the developer work harder to achieve better runtime performance.

The idea of data ownership as a mechanism for determining when its storage can be freed is a new concept for me.

I started my career coding in C. C is a low-level language, where you are exposed to more concepts of the underlying hardware. Are you on a 32-bit or 64-bit machine (because that determines the default size of your integer type)? 

And then Java came along, with syntax similar to C but with garbage collection, so you didn't (usually) need to worry about how the memory was freed. Java code compiles to Java bytecode, and so some of the language constraints are imposed when the source code is compiled to bytecode. And other checks are imposed when you run the code.

And other languages, like Clojure, change the way that you can reason about your programme, while still compiling to the underlying Java bytecode.

Rust requires us to think about how we're allocating data, because the runtime isn't cleaning it up for us. However the Rust compiler prevents us from doing things that would result in undefined behaviour.

[_Consider adding note about [zero-cost abstractions](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/references-and-borrowing.html)._]

The result of this is that you'll likely experience more compile-time errors. _No, you can't do that._ _No, I'm afraid not._ 

These checks make your life as a coder harder. But if highly performant, memory-efficeint, code is what you need for your project then it's worth the investment.

Let's go through some of the ways that Rust may frustrate you as it helps you to write safe code.

We've already established that for types allocated on the heap (which don't implement the _Copy_ trait) **ownership** is moved when the associated variable is assigned, or passed to a function.

Memory for the variable is freed when the owner goes out of scope. 

So far so good. To be able to access those data types without the restrictions of ownership, Rust brings references.

Here are the [rules] around references:

* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

(I don't know yet what the second point means. When might they not be valid?)

Let's have a look at this:

[_These example are from the [Rust book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) - is this OK? Can I come up with better examples?_]

With this code

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1 = s + " world";
    println!("{} {}", s, r1);
}
```

Rust complains:

`cannot borrow `s` as immutable because it is also borrowed as mutable`

_That surprised me, but the compiler error is quite helpful:_

```
let r1 = &mut s;
         ------ mutable borrow occurs here
println!("{} {}", r1, s);
----------------------^-
|                     |
|                     immutable borrow occurs here
mutable borrow later used here
```

Another example:

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{} {}", r1, r2);
}
```

Result: `cannot borrow `s` as mutable more than once at a time`

(Fair enough. That follows from the rules mentioned above.)

Why aren't we allowed multiple mutable references? The Rust docs talk about this preventing a `data race`; a `race condition` is when things can happen in an unpredictable order, for example if the code is using multiple processors on the same machine. If one processor updates a value and another variable depends on that value, how do we keep them consistent? (With difficulty.)

This compiles:

```
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
}
```

Shared immutabe references are OK.

I'm wondering how likely I am to encounter these issues. Or whether the code will still run fast enough if I code in a more immutable way.

[_My thinking is to capture these thoughts now and then return to them later._]





