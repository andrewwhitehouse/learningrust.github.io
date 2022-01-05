**Hello World**

It's customary to write a small "hello world" programme that lets us check that everything's working.

Change directory to where you keep your code. You'll create a sub-directory for your new programme. We're calling ours `helloworld`.

`mkdir helloworld`
`cd helloworld`
`echo // Hello World > main.rs`

We've create a source file with a comment on the first line. Open the file in your favourite editor. You don't have to keep the comment ... it's there to save you from being faced with the void of an empty file.

After the comment add these lines:

```
fn main() {
    println!("Hello, world!");
}
```

Save your file.

Then compile it with `rustc main.rs`

List the files in your directory with `ls -l` and you'll see another file has appeared. Type `file main` and you'll see that it's an executable file.

Run the file with `./main` and you should see the message printed.

Open your file again and replace "world" with your name. Save and recompile. 

Check that the output is what you expect.

Exercises:

1. Which Rust function is called when you execute a compiled programme?
2. What is the preferred file extension for Rust files?
