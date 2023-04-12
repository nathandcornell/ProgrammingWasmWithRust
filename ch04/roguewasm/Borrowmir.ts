import { Being } from "./roguewasm"
import BeingActionHandler from "./BeingActionHandler"

export default class Borrowmir {
  being: Being | undefined
  actionHandler: BeingActionHandler
  character = "B"
  color = "#ff0000"

  constructor(actionHandler: BeingActionHandler) {
    this.actionHandler = actionHandler
  }

  setBeing = (being: Being) => {
    this.being = being
  }

  getX = (): number => {
    if (!this.being) {
      throw new Error('Enemy is missing a being!')
    }

    return this.being.x()
  }
  getY = (): number => {
    if (!this.being) {
      throw new Error('Enemy is missing a being!')
    }

    return this.being.y()
  }
  getCharacter = () => this.character
  getColor = () => this.color

  act = () => {
    this.actionHandler.act(this)
  }
}
