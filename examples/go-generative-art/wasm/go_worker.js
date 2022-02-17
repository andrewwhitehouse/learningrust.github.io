console.log = (line) => {
  postMessage({log: line});
}

self.importScripts("wasm_exec.js");

const go = new self.Go();

let mod, inst;
let result;

WebAssembly.instantiateStreaming(fetch("main.wasm"), go.importObject)
  .then((result) => {
    mod => result.module;
    inst = result.instance;

    go.run(inst);
    console.log("WASM binary loaded");
})
.catch((err) => {
  console.error(err);
});
