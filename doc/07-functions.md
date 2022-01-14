**Functions**

We've already seen some functions in earlier chapters.

```
fn main() {
  println!("Hello World");
}
```

In this example no return value is specified, which means that the return type is implicitly `()`, which is an empty tuple, also known as "unit".

If we want to have a more interesting return type we can do it like this, with last evaluated value returned:

```
fn inc(n: i32) -> i32 {
  n+1
} 

fn main() {
  println!("{}", inc(3));
}
```

You can also return at earlier points:

```
fn foo(n: i32) -> i32 {
  if n > 0 {
    return n * 3;
  } else {
    return n * 2;
  }
}

fn main() {
  println!("{} {}", foo(-23), foo(5));
}
```

Rust has expressions, which have a value, and statements which have "no value" (technically they are of the unit type).

So if you changed the code to look like this:

```
// . . .
} else {
    n * 2;
    let x = n * 2;
}
```

you would get a compiler error. Try it.

Having a single return point can make your programme easier to debug. If you wanted to print the value before returning it you could do this:

```
fn foo(n: i32) -> i32 {
  let ret = if n > 0 { n * 3 } else { n * 2 };
  println!("Foo returns {}", ret);
  ret
}
```

[In case you're wondering what the [foo](https://en.wikipedia.org/wiki/Foobar) it represents an arbitrary identifier whose name isn't really important.]

Some languages support default arguments. Rust does (natively) but you can use Option:

```
fn add(a: i32, maybe_b: Option<i32>, maybe_c: Option<i32>) -> i32{
  let b = if maybe_b.is_none() { 0 } else { maybe_b.unwrap() };
  let c = if maybe_c.is_none() { 0 } else { maybe_c.unwrap() };
  a + b + c
}

fn main() {
  println!("{}", add(3, None, None));
  println!("{}", add(3, Some(8), None));
  println!("{}", add(3, Some(8), Some(6)));
}
```

[_Note: there are better ways to add together a variable number of integers._]

**Recursion**

Recursion involves a function calling itself. It can lead to elegant code in some cases, particularly when dealing with arbitrary hierarchical data structures, like trees.

Here's another contrived example which adds two non-negative numbers by continually increasing the first by 1 and decreasing the second, also by 1.

```
fn overeningeered_add(a: u32, b: u32) -> u32 {
  if b == 0 { a } else { add(a+1, b-1) }
}

fn main() {
  println!("{}", overengineered_add(123, 456));
}
```

Exercises:

1. We previously wrote a [Fibonacci](https://en.wikipedia.org/wiki/Fibonacci_number) function that used an array to return multiple values. We could have simplified it ti use a single value, like this:

   ```
   fn fib_iter(n: usize) -> u64 {
       if n == 0 {
           return 0
       }
       let mut a: u64 = 0;
       let mut b: u64 = 1;
       for _index in 1..n {
           let tmp = b;
           b = a + b;
           a = tmp;
       }
       b
    }

    fn main() {
       for n in 0..=25 {
           println!("{} {}", n, fib_iter(n))
       }
    }
   ```

   Here is a recursive implementation:

   ```
   fn fib_recur(n: usize) -> u64 {
      if n < 2 {
         return n as u64
      }
      // FIXME
      fib_recur(0) + fib_recur(0)
   }

   fn main() {
      for n in 0..=25 {
         println!("{} {}", n, fib(n))
      }
   }
   ```

   Fix the function parameters so that the function correctly prints the first 26 numbers.

2. Rust is quick, but you'll see a noticeable difference between the speed of the iterative and recursive solutions if you use higher values. Compare the run times, starting from around n=40. Which is quicker, and why? 

[details="Answers"]
1. The last line of `fib_recur` should read `fib_recur(n-1) + fib_recur(n-2)`
2. The recursive solution is slower because it repeatedly calculates the same values over and over. 
[/details]
