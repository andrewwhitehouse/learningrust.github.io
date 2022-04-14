**WebAssembly bindgen hello world**

How's that for a snappy title!

I was reflecting on Dale Carnegie's "Don't complain, condemn or criticise". This has been a challenging day!!

I've been attempting to get a "hello world" example working that calls a WASM file -- generated from Rust -- from a JavaScript page.

I was surprised how fiddly this was.

Here are the steps from the Rust book:

I was working on this last night, and then this evening too.

A number of the examples are rather simplistic. They use [primitive types](https://wasmbyexample.dev/examples/hello-world/hello-world.rust.en-us.html). Or [don't pass arguments](https://rustwasm.github.io/book/game-of-life/hello-world.html) to the Rust function, so that's skipping an important proof point. It looks like my book is very close to the [online docs](https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html).

Anyway, this works ...

Here's the example from the book:

`cargo new bindgenhello --lib`

`cd bindgenhello`

Update Cargo.toml

```
[package]
name = "bindgenhello"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

Update src/lib.rs

```
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

Build it

`cargo build --target wasm32-unknown-unknown`

`wasm-bindgen target/wasm32-unknown-unknown/debug/bindgenhello.wasm --out-dir pkg`

(These commands are very easy to mis-type. Note: I specified a sub-directory called pkg to keep the parent free of generate files.)

`$ tree -I wasm32-unknown-unknown -I target`

```
.
├── Cargo.lock
├── Cargo.toml
├── pkg
│   ├── bindgenhello.d.ts
│   ├── bindgenhello.js
│   ├── bindgenhello_bg.js
│   ├── bindgenhello_bg.wasm
│   └── bindgenhello_bg.wasm.d.ts
└── src
    └── lib.rs
```

index.js:

```
const wasm = import('./pkg/bindgenhello');
  
wasm.then(h => h.hello("world!")).catch(console.error);
```    

And it recommends using npm with webpack to build it. I dislike this additional complexity, but I'll go with it for now.

webpack.config.js

```
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development'
};
```

package.json

```
{
  "scripts": {
    "build": "webpack",
    "serve": "webpack-dev-server"
  },
  "devDependencies": {
    "text-encoding": "^0.7.0",
    "html-webpack-plugin": "^3.2.0",
    "webpack": "^4.11.1",
    "webpack-cli": "^3.1.1",
    "webpack-dev-server": "^3.1.0"
  }
}
```

`$ npm run serve` 

produces a popup that says "Hello World!!".

