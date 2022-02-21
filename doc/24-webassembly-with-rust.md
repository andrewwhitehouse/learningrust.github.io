[_Excerpts from the book Programming WebAssembly with Rust. For reference only. Needs to be adapted significantly._]

[_This example demonstrates how to generate a WebAssembly file from Rust, which we can then inspect using wasm-objdump._]

We're going to see how combining Rust with WebAssembly allows us to add strong types, memory safety and more expressive code. 

The Rust language has a longer learning curve than other languages like Go. ***You might want to start skimming through the official Rust book to get familiar with some of the syntax coming up in the book.***

In the history of programming languages, a _genesis moment_ is when a compiler written in that language is able to compile that language. For Rust this came in 2011 and the first stable release was in 2015.

_Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety._

Some of Rust's characteistsics:

- Rust doesn't have the concept of _null_; any data that can be missing is represented with Option.

- Rust will prevent your code from compiling if it could potentially create a data race, free already deallocated resources,or access soething that has gone out of scope.

- Rust has a remarkably expressive syntax and includes many features that people typically laud when talking about functional programming: pattern maching, destructuring, streams, iterators, and much more.

- Rust binaries are standalone, native binaries. They consume relatively little disk space, have a fairly small memory footprint, and the code genreally performs extremely well. Despite all of the additional safety constraints in the language , Rust doesn't need a garbe collector, which can often produce raw C-like performance.

## Installing Rust

If you haven't already, install it using the instructions on the [official website](https://www.rust-lang.org/tools/install).

## Building WebAssembly in Rust

Rust supports compilation to different target machines: Intel, ARM such as Raspberry Pi and WebAssembly.

### Install WebAssembly target

```
$ rustup toolchain list
stable-x86_64-apple-darwin
nightly-x86_64-apple-darwin (default)
$ 
```

Add WebAssembly:

```
$ rustup target add wasm32-unknown-unknown 
info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
$
```

### Create a WebAssembly project

```
$ cargo new --lib rustwasmhello
     Created library `rustwasmhello` package
$ 
```

Creates the following directory structure:

```
.
├── Cargo.toml
└── src
    └── lib.rs
```

Update Catgo.toml:

```
[package]
name = "rustwasmhello"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
```

The `[lib]` section indicates that the project will expose a C-style dynamic library, which is then used by other linker to produce a WebAssembly module.

Update `lib.rs`:

```
#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
```

[_Apparent the "C" part is [optional](https://stackoverflow.com/questions/44664703/whats-the-difference-between-extern-fn-and-extern-c-fn-in-rust)._]

Build the project

`$ cargo build --target wasm32-unknown-unknown`

This produced a rustwasmhello.wasm file under the target/wasm32-unknown-unknown/debug folder. 

You can also build a release version.

`$ cargo build --release --target wasm32-unknown-unknown`

Inspect it with:

`$ wasm-objdump -x target/wasm32-unknown-unknown/release/rustwasmhello.wasm`

(the debug version contains a lot of extra symbols.)

```
$ wasm-objdump -x target/wasm32-unknown-unknown/release/rustwasmhello.wasm

rustwasmhello.wasm:	file format wasm 0x1

Section Details:

Type[1]:
 - type[0] (i32) -> i32
Function[1]:
 - func[0] sig=0 <add_one>
Table[1]:
 - table[0] type=funcref initial=1 max=1
Memory[1]:
 - memory[0] pages: initial=16
Global[3]:
 - global[0] i32 mutable=1 <__stack_pointer> - init i32=1048576
 - global[1] i32 mutable=0 <__data_end> - init i32=1048576
 - global[2] i32 mutable=0 <__heap_base> - init i32=1048576
Export[4]:
 - memory[0] -> "memory"
 - func[0] <add_one> -> "add_one"
 - global[1] -> "__data_end"
 - global[2] -> "__heap_base"
Code[1]:
 - func[0] size=7 <add_one>
Custom:
 - name: ".debug_info"
Custom:
 - name: ".debug_pubtypes"
Custom:
 - name: ".debug_ranges"
Custom:
 - name: ".debug_abbrev"
Custom:
 - name: ".debug_line"
Custom:
 - name: ".debug_str"
Custom:
 - name: ".debug_pubnames"
Custom:
 - name: "name"
 - func[0] <add_one>
 - global[0] <__stack_pointer>
Custom:
 - name: "producers"
$
```