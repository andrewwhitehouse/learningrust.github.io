**Data Types**

Rust is a strongly and statically typed language. This means that the Rust compiler must be able to know the types when the programme is compiled, and type behaviours are enforced more strictly so that programmes are unlikely to suffer from subtle type conversion errors. It also means that you will spend more time pre-emptively fixing these errors than a language that is more relaxed about types.

For more on achieving correctness through types, and other approaches, read Bruce Eckel's article: _Strong Typing versus Strong Testing_.

Here are some examples of primitives, or _scalar_ types, which represent a single value: a number (which can be an integer, without a fractional part), a floating point number, a character, or a boolean value (_true_ or _false_).

Create a boilerplate main function with a helper in a file and directory name of your choosing:

```
use std::any::type_name;
  
#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
  //println!("1 {}", type_of(1));
}
```

When you compile and run the file it won't do anything.

Now uncomment the `println!` call in main.

You should see that the integer literal value `1` defaults to type `i32`.

Add this block and uncomment the line or block comments one by one:

```
  // println!("5i32 {}", type_of(5i32));

  /*
  let max = std::u128::MAX;
  println!("max {} {}", max, type_of(max));
  */

  /*
  let default_integer = 4;
  println!("default_integer {} {}", default_integer, type_of(default_integer));
  */
```

Then do the same for these boolean, character and  floating point examples:

```
  //println!("false {}", type_of(false));

  //println!("\u{2705} {}", type_of('\u{2705}'));

  /*
  let default_float = 3.0;
  println!("default_float {} {}", default_float, type_of(default_float));
  */

  /*
  let a_float: f64 = 1.0;
  println!("a_float {}", type_of(a_float));
  */
```

In this last example, uncomment the block comment, which defines and prints the mutable variable, and then uncomment the line which modified the value. What happens to the type?

```
  /*
  let mut inferred_type = -12; // Type i64 is inferred from another line
  println!("inferred_type (mutable) {} {}", inferred_type, type_of(inferred_type));
  */
  //inferred_type = 5_000_000_000i64;
```

The underscores make it easier to read large numbers. 

_Note: The type suffix may come as a surprise, particurly if you've come from a language like [Java](https://docs.oracle.com/javase/specs/jls/se7/html/jls-5.html). We go into more detail on how to convert between types in a later section._

Exercises:

1. If we knew we didn't need to handle negative numbers, which type should we use instead of `i64`
2. In some cases, Rust uses an architecture-dependent type; what is the type of `[1,2,3].len()` ?
3. What is the naming [convetion](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md) for Rust functions known as?
4. Update your programme to print the Emoji corresponding to Unicode character `1F44F`


[details="Answers"]
1. u64
2. usize
3. snake case
4. :clap: 
[/details]
