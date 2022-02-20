const fs = require('fs');

function loadWasm(filename) {
    const data = fs.readFileSync(filename);
    const wasmSourceCode = new Uint8Array(data);
    const wasmModule = new WebAssembly.Module(wasmSourceCode );
    const wasmInstance = new WebAssembly.Instance(wasmModule);
    return wasmInstance.exports;
}

module.exports = {loadWasm};