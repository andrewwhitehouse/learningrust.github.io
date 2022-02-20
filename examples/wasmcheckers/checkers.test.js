const {loadWasm} = require('./checkers')

describe('loads WASM file', () => {

    it('calculates index for position', () => {
        const exports = loadWasm('checkers.wasm');
        expect(exports.offsetForPosition(3,4)).toBe(140);
    });
});