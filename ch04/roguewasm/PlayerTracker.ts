import Point from "./Point";

export default class PlayerTracker {
  position: Point

  constructor(position: Point) {
    this.position = position
  }

  getPosition(): Point {
    return this.position
  }

  setPosition(position: Point) {
    this.position = position
  }
}
