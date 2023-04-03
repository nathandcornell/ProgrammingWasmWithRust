const START_INDEX: usize = 0;
const END_INDEX: usize = 7;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

pub struct Delta {
    pub x: i8,
    pub y: i8,
}

// Helper for calculating possible moves and jumps
const MOVE_DIRS: [Delta; 4] = [
    Delta { x: -1, y: 0 },
    Delta { x: -1, y: -1 },
    Delta { x: 0, y: 1 },
    Delta { x: 1, y: 1 },
];

#[derive(Clone, Copy, Debug)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }

    pub fn crown(piece: GamePiece) -> GamePiece {
        GamePiece {
            color: piece.color,
            crowned: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn valid(self) -> bool {
        let Coordinate(x, y) = self;
        x <= END_INDEX && y <= END_INDEX
    }

    fn valid_directions(&self, distance: i8) -> Vec<Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x, y) = *self;

        for delta in MOVE_DIRS {
            let new_y: i8 = x as i8 + delta.x * distance;
            let new_x: i8 = y as i8 + delta.y * distance;

            // TODO: We could probably simplify this with some better
            // typecasting to usize (min usize is 0)
            let new_x_valid = START_INDEX as i8 <= new_x && new_x <= END_INDEX as i8;
            let new_y_valid = START_INDEX as i8 <= new_y && new_y <= END_INDEX as i8;

            if new_x_valid && new_y_valid {
                moves.push(Coordinate(new_x as usize, new_y as usize))
            }
        }

        return moves;
    }

    pub fn valid_moves(&self) -> impl Iterator<Item = Coordinate> {
        let moves = self.valid_directions(1);

        moves.into_iter()
    }

    pub fn valid_jumps(&self) -> impl Iterator<Item = Coordinate> {
        let jumps = self.valid_directions(2);

        jumps.into_iter()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        Move {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}
