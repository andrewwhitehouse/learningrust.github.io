**Formatting**

[_I found some examples which better match what I'm trying to do, at https://github.com/rust-lang/rust-by-example - I feel like a fraud copying somebody else's work, but I guess adapting it is OK. I sat earlier thinking about what I was going to write and was thinking to myself "this is too hard", "why have I made this more difficult that it needs to be?". I could choose not to do this, and at my age, with my eyesight not as good as it once was, I wonder if I should leave the new shiny things to the youngsters. But I'm not ready to do that yet. And I do believe that being about to write blockchain smart contracts in a language other than Solidity is a useful thing to be able to do._]

For a programme to do something useful you would expect it to produce some sort out output. This could be saved somewhere in a file, it could be passed to another system, or it could simply print some sort of result. Printing messages is also helpful to print out the state of your programme as it runs, particularly if it isn't doing what you expect.

We saw `println!` being used earlier, and this prints the text to the console (io::stdout) with a newline appended. Use `print!` if you don't want to the newline.

Create a directory for your files.

`mkdir printing`
`cd printing`

Open <code>print.rs</code> in your editor and add the following text.

```
fn main() {
    let centigrade = 3.0
    println!("{} centigrade = {} fahrenheit", centigrate * 1.8 + 32);
}
```

Compile the file with `rustc` and run the executable.

You should see:

`3 centigrade = 37.4 fahrenheit`

You can use positional arguments too.

`"{0} centigrade = {1} fahrenheit"`

Try this.

Exercise:

1. Can you find how to set the precision of the centigrade that's printed so that it appears with one decimal place like this: "3.0 centigrade = 37.4 fahrenheit". Hint: look at the std::fmt docs https://doc.rust-lang.org/std/fmt/, and the precision section, then modify how the formatting arguments are represented.
2. Can you use `format!` to assign the formatted string to a variable first, and then print the string converted to upper case? (Reference: https://doc.rust-lang.org/stable/std/string/struct.String.html#method.to_uppercase)

Copyright (c) 2022 Andrew Whitehouse. All rights reserved.
