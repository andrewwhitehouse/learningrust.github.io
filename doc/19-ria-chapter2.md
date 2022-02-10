**Notes from Rust in Action, chapter 2**

The chapter progresses through these topics in increasing levels of depth:

* primitive types (integers, text ...)
* functions and methods
* control flow (if/else, match and looping)
* complex types (structs and enums)
* collections (vectors, tuples and arrays)
* Rust's utilities (cargo and rustc)
* Project tooling

They share my appreciation for running code. `ok.rs` compiles to the executable file `ok`.

For projects containing more than one file, use `cargo` which understands how to compile source files and combine the result with external dependencies.

`cargo init` initialises the project. `cargo run` runs it. `cargo run -v` (for verbose) tells you what default options it's choosing.

The code sample involves integers defined in different ways (type inferred based on the assigned value, or declared explicitly, or included as an annotation like `30i32` which specifies 30 as a 32-bit signed integer; underscores can be used to enhance readability). There is a simple function definition that adds two integers.

Rust includes a large number of numeric types, so you need to think about the appropriate one for the range of values you need. Conversions between types are always explicit. (Other languages like C and Java support implicit conversion.)

This is C:

```
#include <stdio.h>
int main() {
    long l = 30;
    printf("%ld", l);
}
```

This doesn't compile:

```
fn main() {
   let a: i32 = 30;
   let b: i128 = a;
   println!("{}", b);
}
```

```
error[E0308]: mismatched types
 --> foo.rs:3:18
  |
3 |    let b: i128 = a;
  |           ----   ^ expected `i128`, found `i32`
  |           |
  |           expected due to this
  |
help: you can convert an `i32` to an `i128`
  |
3 |    let b: i128 = a.into();
  |                   +++++++

error: aborting due to previous error
```

Rust's numbers can have methods. It introduces binary and octal notation. 

Rust supports comparison of numeric types with the "standard" operators: <, >, ==, != and >=.

You can't compare variables of different types. You can work around this by casting the smaller type.

```
fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    if a < (b as i32) {
        println!("a is less than b");
    }
}
```

Here's another way of doing the comparison:

```
use std::convert::TryInto;
  
fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("a is less than b");
    }
}
```

If the conversion fails, the call to `unwrap()` results in a programme panic:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: TryFromIntError(())', comparing.rs:7:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Floating point numbers are approximate, because they are represented in base 2 (not base 10). And they have limited precision (in the number of decimal places supported).

So: 1. don't test floating point numbers fo equality, and 2: be wary when results may be mathematically undefined.

```
fn main() {
   let result = 0.1 + 0.2;
   assert!(result == 0.3);
}
```

```
$ ./fp
0.30000000000000004 false
$
```

```
$ ./fp
thread 'main' panicked at 'assertion failed: result == 0.3', fp.rs:3:4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

Operations that produce mathematically undefined results produce `not a number`.

```
fn main() {
    let x = (-42f32).sqrt();
    println!("{}", x);
}
```

```
$ ./nan
NaN
$
```

Two NaN values are never equal.

Guard against undefined values with `is_finite()` and `is_nan()`:

```
fn main() {
    let x = (-42f32).sqrt();
    assert!(x.is_finite());
}
```

```
$ ./nan_check 
thread 'main' panicked at 'assertion failed: x.is_finite()', nan_check.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$
```

[Up to section 2.3.4]
