import { Being, Engine } from "./roguewasm"
import Point from "./Point"
import { Display, Path } from "rot-js"
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

  getX = (): number => this.being.x
  getY = (): number => this.being.y
  getCharacter = () => this.character
  getColor = () => this.color

  act = () => {
    this.actionHandler.act(this)
  }
}
