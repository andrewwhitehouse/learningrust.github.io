**Testing WebAssembly**

This post is guided by the "WebAssembly Checkers" example from the book "Programming WebAssembly with Rust".

This is a more substantial example, implementing the board-game Checkers, or Draughts. As the author says:

> You’ll build this module by creating a series of small functions that, once complete, will work together to provide
the fundamentals of a working checkers game. There’s always a trade-off between the complexity of a real application
and the need to
keep an example simple enough to be used as a learning tool, so we cut a few corners on evaluating some game rules and
edge cases, but the code will be playable when you’re done.

I also want to get a sense for how fragile WebAssembly is; whether the community is accustomed to introduce breaking changes as they did with renaming `get_local` to `local.get` in the last example.

`$ mkdir wasmcheckers`

`$ cd wasmcheckers/`

Create `checkers.wat` (WAT = WebAssembly Text format)

```
(module
  ;; Allocate at leasat one page of 64KB memory
  (memory $mem 1)
) 
```

`$ wat2wasm checkers.wat`

We want to model an 8x8 game board. But WebAssembly doesn't support arrays. It does support linear memory, though ... organise as bytes.

We can map a two-dimensional 8x8 array to a one-dimentional array like this `index = x+y*8`. However there is an extra complication; we're storing 8-bit bytes. So if we want to work with 32-bit integers we need to use 4 bytes for each value.

So the address of each 4-byte value is `(x+y*8)*4`.

I want to create a local JavaScript helper so that I can interact with the code as it's written. Just writing code, even if it compiles, without being able to test it is like fumbling in the dark.

`$ npm init -y`

`$ npm add --save-dev jest`

Modify scripts section in `package.json`:

```
"scripts": {
    "test": "jest"
  },
```

After some Googling I discovered [this post](https://blog.scottlogic.com/2018/04/26/webassembly-by-hand.html) which looks promising. Let's give it a go ...

Actually that post didn't really help. I thrashed about for a while getting errors relating to JavaScript promises and then found a simpler approach.

However I want to deal with the issue of rebuilding the wasm file each time I change the underlying test. The blog post does this in an overly complex way (in my view).

Here's what I ended up with as a way of testing the functions.

**checkers.wat**

```
(module
  ;; Allocate at least one page of 64KB memory
  (memory $mem 1)

  (func $indexForPosition (param $x i32) (param $y i32) (result i32)
    (i32.add
      (i32.mul
        (i32.const 8)
        (local.get $y))
      (local.get $x)))

  ;; Offset = (x+y*8) * 4
  (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
    (i32.mul
      (call $indexForPosition (local.get $x) (local.get $y))
      (i32.const 4)))

  (export "indexForPosition" (func $indexForPosition))
)
```

I modified the book code to use `local.get` and I also added the `export` on the penultimate line so I can call the function externally.

**checkers.test.js**

```
const {loadWasm} = require('./checkers')

describe('loads WASM file', () => {

    it('calculates index for position', () => {
        const exports = loadWasm('checkers.wasm');
        expect(exports.indexForPosition(3,4)).toBe(35);
    });
});
```

**checkers.js**

(helper function.)

```
const fs = require('fs');

function loadWasm(filename) {
    const data = fs.readFileSync(filename);
    const wasmSourceCode = new Uint8Array(data);
    const wasmModule = new WebAssembly.Module(wasmSourceCode );
    const wasmInstance = new WebAssembly.Instance(wasmModule);
    return wasmInstance.exports;
}

module.exports = {loadWasm};
```

To ensure the wasm file is rebuild before running the tests, I changed the package.json

```
"scripts": {
    "test": "wat2wasm checkers.wat && jest"
  },
```

Run the test:

```
$ npm test

> wasmcheckers@1.0.0 test /Users/andrewwhitehouse/code/wasmcheckers
> wat2wasm checkers.wat && jest

 PASS  ./checkers.test.js
  loads WASM file
    ✓ calculates index for position (3 ms)

Test Suites: 1 passed, 1 total
Tests:       1 passed, 1 total
Snapshots:   0 total
Time:        0.391 s, estimated 1 s
Ran all test suites.
$ 
```  

This was a useful exercise. However as I read the rest of the chapter I'm underwhelmed by the result; it's a set of functions which simulate moves on a checker board, applying rules, but with no UI. I'm going to park it here with what I have. The approach to testing should be useful with other examples.


