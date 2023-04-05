mod board;
mod game;

#[macro_use]
extern crate lazy_static;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use mut_static::MutStatic;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> = 
        MutStatic::from(GameEngine::new());
}

/// Exported method for getting the piece at a given location.
#[no_mangle]
pub extern "C" fn get_piece(x: i32, y: i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    let piece = engine.get_piece(Coordinate(x as usize, y as usize));
    match piece {
        Ok(Some(result)) => result.into(),
        Ok(None) => -1,
        Err(_) => -1,
    }
}

/// Exported method for getting the active player.
#[no_mangle]
pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    GamePiece::new(engine.current_turn()).into()
}

/// Exported method for moving a piece.
#[no_mangle]
pub extern "C" fn move_piece(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();
    let movement = Move::new(
        (from_x as usize, from_y as usize),
        (to_x as usize, to_y as usize)
    );
    let result = engine.move_piece(&movement);

    match result {
        Ok(move_result) => {
            unsafe {
                notify_piece_moved(from_x, from_y, to_x, to_y);
            }
            if move_result.crowned {
                unsafe {
                    notify_piece_crowned(to_x, to_y);
                }
            }
            1
        },
        Err(_) => 0,
    }
}

// Imported Notification methods.
extern "C" {
    fn notify_piece_moved(fromX: i32, fromY: i32, toX: i32, toY: i32);
    fn notify_piece_crowned(x: i32, y: i32);
}

const BLACK_FLAG: u8 = 1;
const WHITE_FLAG: u8 = 2;
const CROWN_FLAG: u8 = 4;

/// Converts a GamePiece into a bitmapped integer value.
impl Into<i32> for GamePiece {
    fn into(self) -> i32 {
        let mut val: u8 = 0;

        match self.color {
            PieceColor::Black => val += BLACK_FLAG,
            PieceColor::White => val += WHITE_FLAG,
        }

        if self.crowned {
            val += CROWN_FLAG;
        }

        val as i32
    }
}


