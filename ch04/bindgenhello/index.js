const wasm = import('./build/bindgenhello');

wasm.then(h => h.hello("world!"))
    .catch(console.error);
