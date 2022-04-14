**Integrating WebAssembly with javaScript**

In this chapter you'll take a look at the Rust WebAssembly ecosystem, including the tooling and libraries available to help bridge the gap between WebAseembly and JavaScript. You'll start off by building a new "Hello World" template that illustrates a new way of communicating between JavaScript and Rust.

Next you'll explore tools like wasm_bindgen that allows Rust to see JavaScript classes (and JavaScript to use Rust structures), expose and invokes callbacks in either language, send strings as function parameters, and return complex values, all while maintaining Rust's strict sharing rules. 

By the end of this chapter, you'll not only know the mechanics of how to interoperate with JavaScript, but you'll have seen patterns and examples of when and where you should divide your logic up between JavaScript and Rust WebAssembly modules by building an interactive, browser-based, game.

The techniques shared up to this point have applied generally to to all kinds of WebAssembly applications written in all kinds of languages.

From now on, the discussion will be specific to Rust.

Everything you do after this point will rely on tools and libraries created by the Rust community.

While you have a degree of choice when it comes to JavaScript bindings, the one that you'll be using in this chapter is [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/). This is a combination of a set of _crates_ that support bindings between JavaScript and Rust.

_wasm-bindgen_ injects a bunch of metadaa into your compiled WebAssembly module. Then a separate command line tool reads the metadata, strips it out, and uses the information to generate an appropriate JavaScript "wrapper bridge" containing the kinds of functions, classes, and other primities that the developer wants bound to Rust.

## Install the new tools

Install [NPM](https://docs.npmjs.com/getting-started).

Install wasm-bindgen

```
$ $ cargo install wasm-bindgen-cli
    Updating crates.io index
  Downloaded wasm-bindgen-cli v0.2.79
  Downloaded 1 crate (41.4 KB) in 1.43s
  Installing wasm-bindgen-cli v0.2.79
. . .
Compiling wasm-bindgen-cli-support v0.2.79
   Compiling rouille v3.5.0
   Compiling wasm-bindgen-cli v0.2.79
    Finished release [optimized] target(s) in 2m 06s
   Replacing /Users/andrewwhitehouse/.cargo/bin/wasm-bindgen
   Replacing /Users/andrewwhitehouse/.cargo/bin/wasm-bindgen-test-runner
   Replacing /Users/andrewwhitehouse/.cargo/bin/wasm2es6js
    Replaced package `wasm-bindgen-cli v0.2.42` with `wasm-bindgen-cli v0.2.79` (executables `wasm-bindgen`, `wasm-bindgen-test-runner`, `wasm2es6js`)
$
```

In this section you’ll go through the process of setting up this “Hello, World” piece by piece, and when you’re done,
you’ll have a nice template that you can use as scaffolding to build future projects
(which will come in handy in the second half of this chapter).

Create a new project:

```
$ cargo new bindgenhello --lib
     Created library `bindgenhello` package
$
```

[_22nd February. Still taking notes and running examples from the book, figuring out what to keep and what to leave out ..._]

```
.
├── Cargo.toml
└── src
    └── lib.rs
```

Cargo.toml

```
[package]
name = "bindgenhello"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-binggen = "0.2"
```

src/lib.rs

```
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Import 'window.alert'
#[wasm_bindgen]
extern "C" { 
    fn alert(s: &str);
}

// Export a 'hello' function
#[wasm_bindgen]
pub fn hello(name; &str) {
    alert(&format!("Hello, {}!", name));
}
```

[_Reminder that this is copied verbatim from the book, so can't be used in bulk in my own book. This is how I learn, and process material from books. And is another reason why I think it's valuable to have a stripped down book that includes only the parts that I find useful (... and I believe others would too). I'm adding my own notes and observations in italics._]

Decorating Rust code with #[wasm_bindgen] triggers the invovation of a compile-time Rust macro. Each time the compiler encounters this macro it generates some code on your behalf. Some of it will be code what winds up in your _.wasm_ module, but some if it will be metadata used to help generate the corresponding JavaScript output produced by the `wasm-bindgen` command-line tool.

In `lib.rs` there are two _wasm_bindgen_ bindings. The first binds the `alert` function to the `alert` JavaScript function. With this binding in place, any Rust code that invokes the alert function will be converted into a bunch of code that invokes the JavaScript alert function from inside a WebAssembly module. _Attaching the right `window` context and making all of the JavaScript pieces work properly is done for us by `wasm-bindgen`_.

The second binding exposes the hello function. You've seen before how this kind of function can be exposed. However in this case, it takes a reference to a string as a parameter. We know that string parameters aren't possible in pure WebAssembly, so the generated wrapper code produces the necessary plumbing (in both wast instructions and JavaScript boilerplate) to allow complex data to flow seamlessly between boundaries. [_This is a key point ... pure WebAssembly supports only i32, i64, f32 and f64; there needs to be some extra code to pass strings around._]

Behind the scenes, memory allocation and disposal functions are creates that operate on the module's linear memory. Then, each time `wasm-bindgen` encounters the need for string alocation, the generated JavaScript invokes those functions. In short, all of the hard work you've been doing in the past couple of chapters is now done automatically on your behalf. [_Interesting point about building tension versus making it easier for the reader. I need to remember to include enough tension in the book ... tell them what you're going to tell them, tell them, and tell them what you told them. At the beginning tell the reader what they're going to learn. Include hooks at the end of each chapter on what they're going to build in the next chapter._]

***These wrappers are convenient, of course, but I still firmly believe that you are better off for having learned how things were done "the hard way"*** [_I'm not sure I agree. Give the reader the choice, rather than imposing the author's subjective experience on the reader._]

`$ cargo build --target wasm32-unknown-unknown`

[_That unknown-unknown suffix looks weird. Why is it like that?_]

[_Also, I found two syntax errors when building the code. How can I verify that my code is correct with a particular version of Rust and WebAssembly when including code snippets in the book?_]

Corrected files:

Cargo.toml

```
[package]
name = "bindgenhello"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

lib.rs

```
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Import 'window.alert'
#[wasm_bindgen]
extern "C" { 
    fn alert(s: &str);
}

// Export a 'hello' function
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

[_22nd February pm_]

Build to check that it's valid:

`$ cargo build --target wasm32-unknown-unknown`

Running

`$ wasm-bindgen target/wasm32-unknown-unknown/debug/bindgenhello.wasm --out-dir .`


creates bindgenhello_bg.wasm, bindgenhello_bg.js, bindgenhello_bg.wasm.d.ts and bindgenhello.js in the current directory. 

This is the wrapper function in bindgenhello_bg.js:

```
export function hello(name) {
    var ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.hello(ptr0, len0);
}
```

The memory offset and length of the allocated string are returned from a function called _passStringToWasm_. The function invokes the generated allocation function inside the WebAssembly module, placing the encoded string in the module's linear memory and then doing the relevant pointer arithmetic. Having written your own wast code, you should be able to appreciate how great it is to have this code generated on your behalf.

[_The book also includes a box note about using Cargo Update. "Occasionally when you go to build a project that built successfully the week before, you may see weird errors in libraries that you didn't write. This happens sometimes with conflicts between locally cached builds and libraries pulled from the internet. Most of the time you should be able to resolve this by running cargo update and then attempting the build again. You may also be prompted to update the version if wasm-bindgen used in the CLI and in your .wasm module don't match._]

With the WebAssembly side of this "Hello World" done, it's time to move on to the JavaScript side of the house.

## Integrating with JavaScript and npm

In order to run the sample in a browser you'll need a web server and some way of invokving your script content that invokes the wasm module. 

The author mentions that the real work happens in the index.js; you get simple clean idomatic JavaScript, even though it's using a bridge to interface with a WebAssembly module.

The book example was a bit funky, and involved faffing around with webpack, so I decided not to do that.

Instead I followed this:

https://wasmbyexample.dev/examples/hello-world/hello-world.rust.en-us.html

`cargo install wasm-pack`

`wasm-pack build --target web`

`wasm-bindgen ​​ ​target/wasm32-unknown-unknown/debug/bindgenhello.wasm ​​ ​​--out-dir ​​pkg` ​ 

https://rustwasm.github.io/book/game-of-life/hello-world.html

796  cd bindgenhello/
  797  cargo init --lib
  798  cargo install wasm-pack
  
<screech ...!>  
  
---











  
  
  





