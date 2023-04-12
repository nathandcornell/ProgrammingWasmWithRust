#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Import 'window.alert'
#[wasm_bindgen]
extern "C" {
    fn alert(message: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);

    #[wasm_bindgen(module = "./index.html")]
    fn stats_updated(stats: JsValue);

    pub type Display;

    #[wasm_bindgen(method, structural, js_namespace = ROT)]
    fn draw(this: &Display, x: i32, y: i32, character: &str);

    #[wasm_bindgen(method, structural, js_name = draw, js_namespace = ROT)]
    fn draw_color(this: &Display, x: i32, y: i32, character: &str, color: &str);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct GridPoint {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub hitpoints: i32,
    pub max_hitpoints: i32,
    pub moves: i32,
}

#[wasm_bindgen]
pub struct Being {
    location: GridPoint,
    moves: i32,
    display: Display,
    hitpoints: i32,
    max_hitpoints: i32,
    icon: String,
    color: String,
}

#[wasm_bindgen]
impl Being {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32, icon: &str, color: &str, display: Display) -> Being {
        Being {
            location: GridPoint { x, y },
            display,
            moves: 0,
            max_hitpoints: 100,
            hitpoints: 100,
            icon: icon.to_owned(),
            color: color.to_owned(),
        }
    }

    pub fn x(&self) -> i32 {
        self.location.x
    }

    pub fn y(&self) -> i32 {
        self.location.y
    }

    pub fn draw(&self) {
        &self
            .display
            .draw_color(self.location.x, self.location.y, &self.icon, &self.color);
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.location = GridPoint { x, y };
        self.draw();

        self.moves += 1;
        self.emit_stats();
    }

    pub fn emit_stats(&self) {
        let stats = Stats {
            hitpoints: self.hitpoints,
            max_hitpoints: self.max_hitpoints,
            moves: self.moves,
        };

        let js_value = serde_wasm_bindgen::to_value(&stats);

        stats_updated(js_value.unwrap());
    }

    pub fn take_damage(&mut self, damage: i32) -> i32 {
        self.hitpoints = self.hitpoints - damage;
        self.emit_stats();
        self.hitpoints
    }
}

#[wasm_bindgen]
pub struct GameEngine {
    display: Display,
    points: HashMap<GridPoint, String>,
    prize_location: Option<GridPoint>,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> GameEngine {
        GameEngine {
            display,
            points: HashMap::new(),
            prize_location: None,
        }
    }

    pub fn on_dig(&mut self, x: i32, y: i32, value: i32) {
        if value == 0 {
            let point = GridPoint { x, y };

            self.points.insert(point, ".".to_owned());
        }
    }

    pub fn draw_map(&self) {
        for (key, value) in &self.points {
            self.display.draw(key.x, key.y, &value);
        }
    }

    pub fn redraw_at(&self, x: i32, y: i32) {
        let grid_point = GridPoint { x, y };

        if let Some(cell) = self.points.get(&grid_point) {
            self.display.draw(x, y, cell);
        }
    }

    pub fn place_box(&mut self, x: i32, y: i32) {
        let grid_point = GridPoint { x, y };

        self.points.insert(grid_point, "*".to_owned());
    }

    pub fn open_box(&mut self, player: &mut Being) {
        let [x, y] = [player.x(), player.y()];
        let grid_point = GridPoint { x, y };

        {
            let cell = self.points.get(&grid_point).unwrap();

            if cell != "*" {
                alert("There's no prize box here.");
                return;
            }
        }

        if let Some(ref location) = self.prize_location {
            if *location == grid_point {
                alert("Congratulations! You've found the prize!!");
            } else {
                alert("Uh, oh; it's a trap!");
                player.take_damage(30);
            }
        }

        self.remove_box(grid_point.x, grid_point.y);
    }

    fn remove_box(&mut self, x: i32, y: i32) {
        let location = GridPoint { x, y };

        self.points.insert(location, ".".to_owned());
    }

    pub fn mark_prize(&mut self, x: i32, y: i32) {
        let grid_point = GridPoint { x, y };

        if let Some(cell) = self.points.get(&grid_point) {
            if cell == "*" {
                self.prize_location = Some(grid_point);
            }
        }
    }

    pub fn move_being(&mut self, being: &mut Being, x: i32, y: i32) {
        // replace being icon with what's underneath
        self.redraw_at(being.x(), being.y());

        being.move_to(x, y);
    }

    pub fn free_cell(&self, x: i32, y: i32) -> bool {
        let grid_point = GridPoint { x, y };

        match self.points.get(&grid_point) {
            Some(cell) => cell == "." || cell == "*",
            None => false,
        }
    }
}
