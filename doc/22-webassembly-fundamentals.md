**WebAssembly Fundamentals**

As I read on past the Go example I started last time, it turns out that the wasm runtime is single-threaded an because the example of generative art is CPU bound this creates a problem for running the example. There is a workaround, but it's apparently outside of the scope of the book. :frowning_face: 

>While those functions will indeed execute in different goroutines, the WASM runtime is still single-threaded. While I/O-bound functions will still work in an async fashion, ours are almost entirely CPU-bound. One possible way to solve the issue is to spawn sub-Workers from the main one, which the Go app will post the progress to, but it is very complex, and out of the scope of this book. 

So I'm going back to following the book _Programming WebAssembly with Rust_.

It starts by covering the fundamentals; not generating WebAssembly files from Rust, but coding in the human-readable `wat` format, which looks like LISP.

To get the WebAssembly tools:

1. Install [CMake](https://cmake.org/)

Run the downloaded `.dmg` file (for Mac).

2. Install [wabt](https://github.com/WebAssembly/wabt)

`git clone git@github.com:WebAssembly/wabt.git`

Follow the README ...

`cd wabt`

`git submodule update --init`

```
mkdir build
cd build
cmake ..
cmake --build .
```

```
$ ./wasm2wat --version
1.0.27
$ 
```  
  
```
$ ./wasm-objdump --version
1.0.27
$  
```

Create add1.wat

```
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.add)
  (export "add" (func $add))
)
```

```
$ pushd build
$ export PATH=$PATH:`pwd`
$ popd
```

`​​ wat2wasm ​​ ​​ add1.wat ​​ ​​ -o ​​ ​​ add.wasm ​ `

The example in the book doesn't seem to work:

```
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    get_local $lhs
    get_local $rhs
    i32.add)
  (export "add" (func $add))
)
```

```
$ wat2wasm add1.wat -o add1.wasm
add1.wat:3:5: error: unexpected token get_local, expected ).
    get_local $lhs
    ^^^^^^^^^
add1.wat:4:5: error: unexpected token get_local.
    get_local $rhs
    ^^^^^^^^^
$
```

Turns out that wat [changed](https://github.com/WebAssembly/spec/issues/884) some time in 2019; the book was released in March 2019.

```
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)
  (export "add" (func $add))
)
```

`wat2wasm add1.wat -o add.wasm`

And the version with s-expressions ...

```
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    (i32.add
      (local.get $lhs)
      (local.get $rhs)))
  (export "add" (func $add))
)
```

`wat2wasm add2.wat -o add_sexpr.wasm`

Call it like this:

`(call $add (i32.const 5) (i32.const 9))` 

Inspect it: `wasm-objdump add.wasm -x`

```
$ wasm-objdump add.wasm -x

add.wasm:	file format wasm 0x1

Section Details:

Type[1]:
 - type[0] (i32, i32) -> i32
Function[1]:
 - func[0] sig=0 <add>
Export[1]:
 - func[0] <add> -> "add"
Code[1]:
 - func[0] size=7 <add>
$
```





