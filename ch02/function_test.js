fetch('./build/function_test.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    console.log("Loaded wasm module");
    instance = results.instance;
    console.log("instance: ", instance);

    const WHITE = 2;
    const BLACK = 1;
    const CROWNED_WHITE = 6;
    const CROWNED_BLACK = 5;

    console.log("Calling offset");
    const offset = instance.exports.offsetForPosition(3, 4);
    console.log(`Offset for position (3, 4) is ${offset}`);

    console.debug("White is white?", instance.exports.isWhite(WHITE));
    console.debug("White is black?", instance.exports.isBlack(WHITE));
    console.debug("Black is black?", instance.exports.isBlack(BLACK));
    console.debug("Black is white?", instance.exports.isWhite(BLACK));

    console.debug("White is uncrowned?", instance.exports.unCrowned(WHITE));
    console.debug("Black is uncrowned?", instance.exports.unCrowned(BLACK));
    console.debug("Crowned Black is uncrowned?", instance.exports.unCrowned(CROWNED_BLACK));
    console.debug("Crowned White is uncrowned?", instance.exports.unCrowned(CROWNED_WHITE));

    console.debug("Crowned Black is crowned?", instance.exports.crowned(CROWNED_BLACK))
    console.debug("Uncrowned Black is crowned?", instance.exports.crowned(BLACK))
    console.debug("Crowned White is crowned?", instance.exports.crowned(CROWNED_WHITE))
    console.debug("Uncrowned White is crowned?", instance.exports.crowned(WHITE))
  });
