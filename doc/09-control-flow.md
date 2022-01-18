**Control Flow - Loops**

Often you will want to repeat an operation until a particular condition is met. There are a few different ways to do this.

While loops look like this:

```
while _condition_ {
  // body
}
```

Here is an example implementation of [Euclid's algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm) to find the gretest common divisor.

```
fn gcd(a: u64, b: u64) -> u64 {
    // Shadow parameters with mutable variables
    let mut b = b;
    let mut a = a;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
  println!("{}", gcd(10, 2));
}
```

We can also implement this as a loop:

```
fn gcd(a: u64, b: u64) -> u64 {
    // Shadow parameters with mutable variables
    let mut b = b;
    let mut a = a;
    loop {
        let temp = b;
        b = a % b;
        a = temp;
        if b == 0 {
            break a;
        }
    }
}
```

In this example, the value passed to break becomes the value of the loop, which is the last expression evaluated in the function.

You can loop through the elements of array:

```
fn main() {
    let mut sum = 0;
    let values = [1,3,5,8,17];
    for v in values {
        sum += v;
    }
    println!("{}", sum);
}
```

or access them by their index

```
fn main() {
    let values = [1,3,5,8,17];
    let mut sum = 0;
    for index in 0..values.len() {
        sum += values[index];
    }
    println!("{}", sum);
}
```

Exercise:

1. You can simplify a fraction by dividing the numerator (top) and denominator (bottom) by the greatest common divisor. 

Here's a function template using tuples that you can use (or ignore):

```
fn simplify((numerator, denominator): (u64, u64)) -> (u64, u64) {
  
}
```

What is `357 / 1411` in its simplest form?

[details="Answer"]

```
fn simplify((numerator, denominator): (u64, u64)) -> (u64, u64) {
    let mut b = numerator;
    let mut a = denominator;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    let gcd = a;
    (numerator / gcd, denominator / gcd)
}

fn main() {
  println!("{:?}", simplify((357, 1411)));
}
```

357/1411 simplified is 31/83

[/details]
