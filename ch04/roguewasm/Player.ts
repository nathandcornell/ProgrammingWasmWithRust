import { Being } from "./roguewasm"
import Point from "./Point"
import { Display } from "rot-js"
import BeingActionHandler, { ActionType } from "./BeingActionHandler"

export default class Player {
  being: Being | undefined
  actionHandler: BeingActionHandler
  character = "@"
  color = "#ff0"

  constructor(actionHandler: BeingActionHandler) {
    this.actionHandler = actionHandler
  }

  setBeing = (being: Being) => {
    this.being = being
  }

  getX = (): number => {
    if (!this.being) {
      throw new Error('Player is missing a being!')
    }

    return this.being.x()
  }
  getY = (): number => {
    if (!this.being) {
      throw new Error('Player is missing a being!')
    }

    return this.being.y()
  }
  getCharacter = () => this.character
  getColor = () => this.color

  act = () => {
    this.actionHandler.act(this)
  }

  handleEvent = () => {
  }
}
