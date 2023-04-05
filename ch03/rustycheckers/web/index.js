fetch('./build/rustycheckers.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {
    env: {
      notify_piece_moved: (fromX, fromY, toX, toY) => {
        console.log(`A piece_moved from (${fromX}, ${fromY}) to (${toX}, ${toY})`);
      },
      notify_piece_crowned: (x, y) => {
        console.log(`A piece was crowned at (${x}, ${y})`);
      }
    }
  }))
  .then(results => {
    instance = results.instance;

    const getCurrentTurn = instance.exports.get_current_turn;
    const move = instance.exports.move_piece;
    const getPiece = instance.exports.get_piece;

    console.log(`Starting the game with player ${getCurrentTurn()}`);

    let piece = getPiece(0, 7);
    console.log(`Piece at (0, 7) is ${piece}`);

    let moveResult = move(0, 5, 1, 4); // Black move
    console.log(`Move result: ${moveResult}`);
    console.log(`New turn for player ${getCurrentTurn()}`);

    let illegalMoveResult = move(1, 4, 2, 3)
    console.log(`Illegal move result: ${illegalMoveResult}`)
    console.log(`After illegal move, turn for player ${getCurrentTurn()}`);

    // Finally, move the crowned piece:
    const result = move(0, 0, 0, 2);

    document.getElementById("container").innerText = result;

    console.log(`At the game end, it is player ${getCurrentTurn()}'s move`)
  })
  .catch(console.error);
