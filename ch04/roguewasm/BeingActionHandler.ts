import { DIRS, Display, Engine as RotEngine, Path } from "rot-js"
import Borrowmir from "./Borrowmir"
import { GameContext } from "./Game"
import Player from "./Player"
import PlayerTracker from "./PlayerTracker"
import Point from "./Point"
import { Being, GameEngine } from "./roguewasm"

export enum ActionType {
  Move,
  Dig
}

export default class BeingActionHandler {
  display: Display
  gameEngine: GameEngine
  rotEngine: RotEngine
  window: Window
  playerTracker: PlayerTracker

  OPEN_KEYCODES = [13, 32]
  MOVE_KEYCODES = [38, 33, 39, 34, 40, 35, 37, 36]
  VALID_KEYCODES = this.OPEN_KEYCODES.concat(this.MOVE_KEYCODES)
  KEYS_TO_DIRS = this.MOVE_KEYCODES.reduce((acc: Map<number, number>, value: number, index: number) => {
    return acc.set(value, index)
  }, new Map<number, number>())

  constructor(display: Display, gameEngine: GameEngine, rotEngine: RotEngine, window: Window, playerTracker: PlayerTracker) {
    this.display = display
    this.gameEngine = gameEngine
    this.rotEngine = rotEngine
    this.window = window
    this.playerTracker = playerTracker
  }

  act(actor: Borrowmir | Player) {
    if (actor instanceof Player) {
      this.rotEngine.lock();
      this.window.addEventListener("keydown", this.getKeydownHandler(actor));
    } else {
      this.moveEnemy(actor)
    }
  }

  moveEnemy(enemy: Borrowmir) {
    const playerPosition = this.playerTracker.getPosition()
    const x = playerPosition.x
    const y = playerPosition.y
    let path: Point[] = [];

    const passCallback = (x: number, y: number) => {
      return this.gameEngine.free_cell(x, y)
    }

    const pathCallback = (x: number, y: number) => {
      path.push(new Point(x, y))
    }

    const pathFinder = new Path.AStar(x, y, passCallback, { topology: 4 })

    pathFinder.compute(enemy.being.x(), enemy.being.y(), pathCallback)

    let nextMove = path.shift()

    while (nextMove) {
      let x = nextMove.x
      let y = nextMove.y

      this.moveActor(enemy, x, y);
    }

    this.rotEngine.lock();
    this.window.alert("Game over - you were captured by the Borrow Checker!!");
  }

  openBox(player: Player) {
    this.gameEngine.open_box(player.being)
  }

  movePlayer(player: Player, keyCode: number) {
    const directionIndex = this.KEYS_TO_DIRS.get(keyCode)

    if (directionIndex === undefined) { return }

    const [xDelta, yDelta] = DIRS[8][directionIndex]
    const newX = player.getX() + xDelta
    const newY = player.getY() + yDelta

    return this.moveActor(player.being, newX, newY)
  }

  moveActor(actor: Player | Borrowmir, x: number, y: number) {
    const being = actor.being

    if (!this.gameEngine.free_cell(x, y)) { return }

    this.gameEngine.move_player(being, x, y)
  }

  getKeydownHandler(player: Player): (event: KeyboardEvent) => void {
    const keyListener = (event: KeyboardEvent) => {
      const keyCode = event.keyCode

      if (!(keyCode in this.VALID_KEYCODES)) { return; }

      // If the keypress is our "open" command, just do that and quit
      if (keyCode in this.OPEN_KEYCODES) {
          this.openBox(player)
      } else {
        // Otherwise, evaluate the movement
        this.movePlayer(player, keyCode)
      }

      window.removeEventListener("keydown", keyListener);

      this.rotEngine.unlock();
    }

    return keyListener
  }
}
