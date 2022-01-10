**Compounds Types - Arrays**

Arrays in Rust have a fixed length, and are allocated on the stack. A related type is the _vector_ which can grow or shrink dynamically.

If we define an array for the number of days in each month, it might look like this for a non-leap year:

```
let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
```

or, with the type specific explicitly:

```
let days_in_month: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
```

The array type is specified as: ```[<element type>; <number of elements]```

Let's do something useful with this data.

This code populates an array with Fibonacci numbers.

```
fn fibonacci() -> [u64; 90] {
    let mut fib = [0; 90];
    fib[0] = 0;
    fib[1] = 1;
    for index in 2..fib.len() {
        fib[index] = fib[index-1] + fib[index-2];
    }
    return fib;
}
```

Note: this approach illustrates populating multiple values in an array, and suits our example. If you knew you wanted to return only a single value, using an array may be unnecessary. 

Exercises:

1. The initial entries in the array are 0,1,1,2,3,5,8,... so fib[6] = 8. What is fib[75] + fib[68] ?
2. What is the ratio of the last element divided by its previous element?
3. What is fib[105]? (_You'll need to do more than simply change the array size._)

[details="Answer"]
1. 2184208538226191
2. 1.61803398875; the ratio of consecutive elements in the Fibonacci sequence tends towards this number, known as the [Golden Ratio](https://en.wikipedia.org/wiki/Golden_ratio) which has been studied for a long time and appears in art, architecture and nature.
3. 3928413764606871165730
[/details]   



