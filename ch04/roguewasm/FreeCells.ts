import Point from "./Point"

import { RNG as RandomNumberGenerator } from "rot-js"

export default class FreeCells {
  cells: Point[]

  constructor(cells?: Point[]) {
    this.cells = cells || []
  }

  push(cell: Point) {
    this.cells.push(cell)
  }

  popRandomCell(): Point {
    if (this.cells.length < 1) {
      throw new Error('No free points remain!')
    }

    const index = Math.floor(RandomNumberGenerator.getUniform() * this.cells.length)
    const randomCell = this.cells[index]
    this.cells.splice(index, 1)

    return randomCell
  }
}
