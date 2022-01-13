**Control Flow (and Tests)**

We already covered some control flow with the if expression. Let's expand on that.

And if you find yourself writing code regularly, you will likely write some tests for your code. Tests are useful because they show you that your code is still behaving correctly as it evolves over time. So if you write some code that you want to be used for a long time, it's a very good idea to write tests.

If we write the tests first it allows us to think about the behaviour we're looking for from our code, or functions. So let's try that. This technique is known as _test-driven development_. 

And we're going to apply it to a coding challenge called _fizz buzz_. Both this challenge, and the Fibonacci generator have shown up as interview questions.

The rules for fizz buzz are as follows:

Given a positive number:
1. If it is a multiple of 3, print "fizz".
2. If it is a multiple of 5, print "buzz"
3. If it is a multiple of both 3 and 5, print "fizzbuzz"

Using the `cargo` tool gives us a pre-made project structure that we can work with.

`cargo new fizzbuzz --lib`

`cd fizzbuzz`

The project looks like this:

```
.
├── Cargo.toml
└── src
    └── lib.rs
```

And `lib.rs` is populated with a starter test.

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
````

The process for [test-driven development](https://en.wikipedia.org/wiki/Test-driven_development) is: write a failing test > run all tests (new test should fail) > make the simplest change for it to pass > ensure all tests pass > refactor.

And repeat.

Let's go ...

Update lib.rs with a failing test.

```
pub fn fizzbuzz(n: u64) -> String {
  "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_as_a_string() {
        assert_eq!(fizzbuzz(1), "1");
    }
}
```

`cargo test` should report (among other things):

```
thread 'tests::it_returns_the_as_a_string' panicked at 'assertion failed: `(left == right)`
  left: `""`,
 right: `"1"`', src/lib.rs:11:9
```

We chose our first test case as 1, because we know we're dealing with positive numbers. Our function returns a string, so when we pass it 1, we should receive "1". There are various values we could have chosen to make it fail, including "0" ... I opted for an empty string (which isn't a valid number, or one of the other strings.

Now let's fix the test.

Replace 

`"".to_string()` with `"1".to_string()` and re-run `cargo test` which should now report a passing test.

Now let's add the next case: "2".

```
#[test]
  fn it_returns_two_as_a_string() {
      assert_eq!(fizzbuzz(2), "2");
  }
```

Run the tests, to ensure they fail. Going through the failing step is important, because when you make the test pass you will know it was your change that made it pass. 

_If you went from a passing test to another passing test, how do you know that your change actually contributed to the additional test passing?_

Fix the implementation:

```
if n == 1 { "1".to_string() } else { "2".to_string() }
```

Failing test:

```
    #[test]
    fn it_returns_fizz_for_3() {
        assert_eq!(fizzbuzz(3), "fizz");
    }
```

Make it pass:

```
pub fn fizzbuzz(n: u64) -> String {
  if n == 3 { return "fizz".to_string() }
  if n == 1 { "1".to_string() } else { "2".to_string() }
}
```

Refactor:

```
pub fn fizzbuzz(n: u64) -> String {
  if n == 3 { "fizz".to_string() } else { n.to_string() }
}
```

Let's add a "buzz" test case:

```
#[test]
    fn it_returns_buzz_for_5() {
        assert_eq!(fizzbuzz(5), "buzz");
    }
```

Make it pass:

```
pub fn fizzbuzz(n: u64) -> String {
    if n == 3 {
        "fizz".to_string()
    } else if n == 5 {
        "buzz".to_string()
    } else {
        n.to_string()
    }
}
```

(If you want to ensure that your code is formatted correctly, run `cargo fmt`.)

What about six?

```
 #[test]
    fn it_returns_fizz_for_6() {
        assert_eq!(fizzbuzz(6), "fizz");
    }
```

We need to check that the number is a multiple of three:

```
if n % 3 == 0 
```

Ten ...

```
#[test]
    fn it_returns_buzz_for_10() {
        assert_eq!(fizzbuzz(10), "buzz");
    }
```

(Test should fail.)

```
} else if n % 5 == 0 {
```

Let's do a small refactoring:

```
pub fn fizzbuzz(n: u64) -> String {
    let is_multiple_of_3 = n % 3 == 0;
    if is_multiple_of_3 {
        "fizz".to_string()
    } else if n % 5 == 0 {
        "buzz".to_string()
    } else {
        n.to_string()
    }
}
```

(Check that the tests still pass.)

Exercise:

1. Add the test case for 15
   ```
    #[test]
    fn it_returns_fizzbuzz_for_15() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
    }
   ```

then check that a single test fails, before fixing it. You may find it helpful to refactor your code as I did for 3, to prevent the same calculation (of division modulo 5) being repeated.

Make the test pass.

Then add another test case to ensure that your code returns "fizzbuzz" for more than just 15. What is the next number that is a multiple of both 3 and 5?

[details="Answers"]
1. Example implementation:
   
```
 pub fn fizzbuzz(n: u64) -> String {
    let is_multiple_of_3 = n % 3 == 0;
    let is_multiple_of_5 = n % 5 == 0;
    if is_multiple_of_3 && is_multiple_of_5 {
        "fizzbuzz".to_string()
    } else if is_multiple_of_3 {
        "fizz".to_string()
    } else if is_multiple_of_5 {
        "buzz".to_string()
    } else {
        n.to_string()
    }
}
```

```
// Additional tests ...

#[test]
fn it_returns_fizzbuzz_for_15() {
      assert_eq!(fizzbuzz(15), "fizzbuzz");
}

#[test]
fn it_returns_fizzbuzz_for_30() {
      assert_eq!(fizzbuzz(30), "fizzbuzz");
}
````

[/details]

[_I wonder if this had enough control flow ... I'm not sure. I think the TDD is helpful though. And now it's done we can dip in and out as needed later. TDD is good, but it's possible to be too dogmatic. I'm still getting my head around the use of Strings; those repeated `to_string()` calls don't look great to me. And I found a thread about someone's [frustrations](https://www.reddit.com/r/rust/comments/rylrma/im_losing_hope_to_ever_learn_this_language/) learning Rust. These typed languages can feel like they are getting in your way._]
