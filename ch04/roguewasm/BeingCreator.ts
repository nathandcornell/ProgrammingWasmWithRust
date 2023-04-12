import { Being, GameEngine } from "./roguewasm"
import Borrowmir from "./Borrowmir"
import Player from "./Player"
import { Display } from "rot-js"
import FreeCells from "./FreeCells"
import BeingActionHandler from "./BeingActionHandler"

export default class BeingCreator {
  display: Display
  gameEngine: GameEngine
  actionHandler: BeingActionHandler

  constructor(display: Display, gameEngine: GameEngine, actionHandler: BeingActionHandler) {
    this.display = display
    this.gameEngine = gameEngine
    this.actionHandler = actionHandler
  }

  createPlayer(freeCells: FreeCells): Player {
    const player = new Player(this.actionHandler)

    const coordinates = freeCells.popRandomCell()
    const being = new Being(coordinates.x, coordinates.y, player.getCharacter(), player.getColor(), this.display)
    being.draw()
    player.setBeing(being)

    return player
  }

  createEnemy(freeCells: FreeCells): Borrowmir {
    const enemy = new Borrowmir(this.actionHandler)

    const coordinates = freeCells.popRandomCell()
    const being = new Being(coordinates.x, coordinates.y, enemy.getCharacter(), enemy.getColor(), this.display)
    being.draw()
    enemy.setBeing(being)

    return enemy
  }
}
