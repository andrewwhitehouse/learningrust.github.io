const wasm = import('./pkg/bindgenhello');

wasm.then(h => h.hello("world!")).catch(console.error);
