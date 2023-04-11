import { GameEngine } from "./roguewasm"
import { Display, Engine as RotEngine, Scheduler } from "rot-js"
import Game, { Dimensions, GameContext } from "./Game"
import BeingCreator from "./BeingCreator"
import FreeCells from "./FreeCells"
import BeingActionHandler from "./BeingActionHandler"
import PlayerTracker from "./PlayerTracker"
import Point from "./Point"

const WINDOW_WIDTH: number = 125
const WINDOW_HEIGHT: number = 40

export type Stats = {
  hitPoints: number
  maxHitpoints: number
  moves: number
}

export function stats_updated(stats: Stats) {
  let hitPoints = document.getElementById("hitpoints")
  let maxHitpoints = document.getElementById("max_hitpoints")
  let moves = document.getElementById("moves")

  if (!hitPoints || !maxHitpoints || !moves) {
    return
  }

  hitPoints.textContent = stats.hitPoints.toString()
  maxHitpoints.textContent = stats.maxHitpoints.toString()
  moves.textContent = stats.moves.toString()
}

const runGame = () => {
  const dimensions = { height: WINDOW_WIDTH, width: WINDOW_WIDTH }
  const display = new Display({width: dimensions.width, height: dimensions.height})
  const gameEngine = new GameEngine(display)
  const scheduler = new Scheduler.Simple();
  const rotEngine = new RotEngine(scheduler);
  const playerTracker = new PlayerTracker(new Point(0, 0))
  const actionHandler = new BeingActionHandler(display, gameEngine, rotEngine, window, playerTracker)
  const creator = new BeingCreator(display, gameEngine, actionHandler)


  const container = display.getContainer()
  const canvasId = "rogue-canvas"
  const canvasElement = document.getElementById(canvasId)
  const context: GameContext = {
    gameEngine,
    freeCells: new FreeCells(),
    rotEngine,
    scheduler
  }
  const game = new Game(gameEngine, creator, playerTracker, dimensions)

  if (!container) {
    console.error("Failed to instantiate display container")
    return
  }

  if (!canvasElement) {
    console.error("Couldn't find ${canvasId} element")
    return
  }

  canvasElement.appendChild(container)
  game.start()
}

runGame()
