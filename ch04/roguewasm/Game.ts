import BeingCreator from "./BeingCreator"
import Borrowmir from "./Borrowmir"
import Player from "./Player"
import PlayerTracker from "./PlayerTracker"
import FreeCells from "./FreeCells"
import Point from "./Point"

import { Engine as RotEngine, Map, Scheduler } from "rot-js"
import { GameEngine } from "./roguewasm"

export type Dimensions = {
  height: number
  width: number
}

export type GameContext = {
  freeCells: FreeCells,
  gameEngine: GameEngine,
  rotEngine: RotEngine,
  scheduler: Scheduler
}

export default class Game {
  context: GameContext
  creator: BeingCreator
  dimensions: Dimensions
  enemy?: Borrowmir
  player?: Player
  playerTracker: PlayerTracker

  constructor(context: GameContext, creator: BeingCreator, playerTracker: PlayerTracker, dimensions: Dimensions) {
    this.context = context
    this.creator = creator
    this.playerTracker = playerTracker
    this.dimensions = dimensions
  }

  generateMap = (): void => {
    const digger = new Map.Digger(this.dimensions.height, this.dimensions.width)

    const digCallback = (x: number, y: number, value: number) => {
      if (!value) {
        var key = new Point(x, y)
        this.context.freeCells.push(key)
      }

      this.context.gameEngine.on_dig(x, y, value)
    }

    digger.create(digCallback.bind(this))

    this.generateBoxes()
    this.context.gameEngine.draw_map()

    this.player = this.creator.createPlayer(this.context.freeCells)

    const playerPosition = new Point(this.player.getX(), this.player.getY())
    this.playerTracker.setPosition(playerPosition)

    this.enemy = this.creator.createEnemy(this.context.freeCells)
  }

  generateBoxes = (): void => {
    for (let i = 0; i < 10; i++) {
      const randomCell = this.context.freeCells.popRandomCell()

      this.context.gameEngine.place_box(randomCell.x, randomCell.y);

      if (i == 9) {
        this.context.gameEngine.mark_wasmprize(randomCell.x, randomCell.y);
      }
    }
  }

  start = () => {
    this.generateMap()

    this.context.scheduler.add(this.player, true);
    this.context.scheduler.add(this.enemy, true);

    this.context.rotEngine.start();
  }
}
