fetch('./build/checkers.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {
    events: {
      piece_moved: (fromX, fromY, toX, toY) => {
        console.log(`A piece_moved from (${fromX}, ${fromY}) to (${toX}, ${toY})`);
      },
      piece_crowned: (x, y) => {
        console.log(`A piece was crowned at (${x}, ${y})`);
      }
    }
  }))
  .then(results => {
    instance = results.instance;

    const getTurnOwner = instance.exports.getTurnOwner;
    const init = instance.exports.initBoard;
    const move = instance.exports.move;

    init();
    console.log(`Starting the game with player ${getTurnOwner()}`);

    move(0, 5, 0, 4); // Black move
    move(1, 0, 1, 1); // White move
    move(0, 4, 0, 3); // Black move
    move(1, 1, 1, 0); // White move
    move(0, 3, 0, 2); // Black move
    move(1, 0, 1, 1); // White move
    move(0, 2, 0, 0); // Black move, coronation event
    move(1, 1, 1, 0); // White move

    // Finally, move the crowned piece:
    const result = move(0, 0, 0, 2);

    document.getElementById("container").innerText = result;

    console.log(`At the game end, it is player ${getTurnOwner()}'s move`)
  })
  .catch(console.error);
