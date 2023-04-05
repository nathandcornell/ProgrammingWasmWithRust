use super::board::{Coordinate, GamePiece, Move, PieceColor, END_INDEX, START_INDEX};
use std::collections::HashMap;

/// The GameEngine, which tracks our state and interactions
pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
    valid_moves: HashMap<PieceColor, Vec<Move>>,
}

/// A Result object for our game
pub struct MoveResult {
    pub movement: Move,
    pub crowned: bool,
}

impl GameEngine {
    /// ******************
    /// * Public Methods *
    /// ******************

    /// Constructor
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
            valid_moves: HashMap::new(),
        };

        engine.initialize();
        engine
    }

    /// Initializes the game.
    ///
    /// Sets the pieces on the board, and generates the initial set of valid
    /// moves.
    pub fn initialize(&mut self) {
        const EVEN_SPACES: [usize; 4] = [0, 2, 4, 6];
        const ODD_SPACES: [usize; 4] = [1, 3, 5, 7];

        // Black rows are indexed 5, 6, and 7. White are 0, 1, and 2.
        // Create a y value for each of the 4 placements on each row:
        let black_rows: [usize; 12] = [[7; 4], [6; 4], [5; 4]].concat().try_into().unwrap();
        let white_rows: [usize; 12] = [[0; 4], [1; 4], [2; 4]].concat().try_into().unwrap();

        // Black pieces sit on two rows of even x coordinates, with one row of
        // odd coordinates between them. White is the opposite.
        let black_spaces: [usize; 12] = [EVEN_SPACES, ODD_SPACES, EVEN_SPACES]
            .concat()
            .try_into()
            .unwrap();
        let white_spaces: [usize; 12] = [ODD_SPACES, EVEN_SPACES, ODD_SPACES]
            .concat()
            .try_into()
            .unwrap();

        // Zip the x coordinates from the '_spaces' arrays with the y
        // coordinates from the '_rows' arrays, creating (x, y) tuples for
        // every pair:
        let black_placements = black_spaces.iter().zip(black_rows.iter());
        let white_placements = white_spaces.iter().zip(white_rows.iter());

        // Set the white pieces:
        white_placements.for_each(|(x, y)| {
            self.board[*x][*y] = Some(GamePiece::new(PieceColor::White));
        });

        // Set the black pieces:
        black_placements.for_each(|(x, y)| {
            self.board[*x][*y] = Some(GamePiece::new(PieceColor::Black));
        });

        self.valid_moves = self.init_valid_moves()
    }

    /// Move a piece.
    ///
    /// Ensures that a move is valid for the given color, direction, jump, etc.
    /// Handles the mutation of the board, including marking the 'from' space
    /// empty, marking the 'to' space with the correct piece, and emptying any
    /// jumped spaces.
    /// Regenerates the valid moves for the mutated spaces on the board.
    /// Finally, toggles the active player and increments the move counter.
    pub fn move_piece(&mut self, movement: &Move) -> Result<MoveResult, ()> {
        let valid_move_list = match self.valid_moves.get(&self.current_turn) {
            None => return Err(()),
            Some(list) => list,
        };

        if !valid_move_list.contains(movement) {
            return Err(());
        }

        let Coordinate(from_x, from_y) = movement.from;
        let piece = self.board[from_x][from_y].unwrap();

        let jumped_piece_coords = self.jumped_piece_coords(movement.from, movement.to);

        if let Some(Coordinate(x, y)) = jumped_piece_coords {
            let jumped_piece = self.board[x][y].unwrap();
            self.remove_piece(Coordinate(x, y), jumped_piece);
        }

        // Move the piece at the "from" coordinates to the "to" coordinates:
        self.set_piece(movement.to, piece);
        self.remove_piece(movement.from, piece);

        let crowned = if self.should_crown(piece, movement.to) {
            self.crown(movement.to);
            true
        } else {
            false
        };

        self.advance_turn();

        Ok(MoveResult {
            movement: *movement,
            crowned,
        })
    }

    /// Returns the color playing the current turn
    pub fn current_turn(&self) -> PieceColor {
        self.current_turn
    }

    /// Returns the number of moves
    pub fn move_count(&self) -> u32 {
        self.move_count
    }

    /* *****************
     * Private Methods *
     *******************/

    /// Returns all possible moves for a given location.
    ///
    /// Does not evaluate any game-state specific rules e.g. crown, color, etc.
    fn valid_moves_for(&self, location: Coordinate) -> Vec<Move> {
        let mut moves = location
            .valid_moves()
            .map(|ref to| Move {
                from: location,
                to: *to,
            })
            .collect::<Vec<Move>>();

        let mut jumps = location
            .valid_jumps()
            .map(|ref to| Move {
                from: location,
                to: *to,
            })
            .collect::<Vec<Move>>();

        moves.append(&mut jumps);

        moves
    }

    /// Initialize all possible and legal moves for the current game state.
    fn init_valid_moves(&mut self) -> HashMap<PieceColor, Vec<Move>> {
        let mut map: HashMap<PieceColor, Vec<Move>> = HashMap::new();

        const RANGE_END: usize = END_INDEX + 1;

        for y in START_INDEX..RANGE_END {
            for x in START_INDEX..RANGE_END {
                if let Some(piece) = self.board[x][y] {
                    let location = Coordinate(x, y);

                    let mut location_moves = self.valid_moves_for(location);
                    let mut legal_moves = self.legal_moves(piece, location_moves);

                    map.entry(piece.color)
                        .and_modify(|color_list| color_list.append(&mut legal_moves))
                        .or_insert(legal_moves);
                }
            }
        }
        map
    }

    /// Filters a given set of moves according to game state.
    ///
    /// Evaluates move direction rules (e.g. color+crown rules), destination
    /// rules (to avoid collisions), and jump rules (jumped piece colors)
    fn legal_moves(&mut self, piece: GamePiece, moves: Vec<Move>) -> Vec<Move> {
        return moves
            .iter()
            .copied()
            // Filter out crowned-only moves unless crowned:
            .filter(|mve| self.valid_direction(piece, *mve))
            // Filter out moves that collide with another piece:
            .filter(|mve| self.valid_destination(*mve))
            // Filter out jumps that don't pass over another piece:
            .filter(|mve| !self.is_jump(*mve) || self.valid_jump(piece, *mve))
            .collect::<Vec<Move>>();
    }

    /// Returns whether a given move direction is valid.
    ///
    /// Evaluates the piece color and crown state.
    fn valid_direction(&self, piece: GamePiece, movement: Move) -> bool {
        if piece.crowned { return true }

        let Coordinate(_from_x, from_y) = movement.from;
        let Coordinate(_to_x, to_y) = movement.to;
        let y_delta: i8 = to_y as i8 - from_y as i8;

        match piece.color {
            PieceColor::Black => y_delta < 0,
            PieceColor::White => y_delta > 0,
        }
    }

    /// Returns whether a given move is a jump or not.
    fn is_jump(&self, movement: Move) -> bool {
        let Coordinate(_from_x, from_y) = movement.from;
        let Coordinate(_to_x, to_y) = movement.to;
        let y_delta: i8 = to_y as i8 - from_y as i8;

        y_delta == 2 || y_delta == -2
    }

    /// Returns whether a given jump is valid or not.
    ///
    /// Evaluates the piece color, and the color of the jumped piece.
    fn valid_jump(&self, piece: GamePiece, movement: Move) -> bool {
        let Coordinate(_from_x, from_y) = movement.from;
        let Coordinate(_to_x, to_y) = movement.to;
        
        match self.jumped_piece_coords(movement.from, movement.to) {
            None => false,
            Some(location) => {
                let Coordinate(jumped_x, jumped_y) = location;

                match self.board[jumped_x][jumped_y] {
                    None => false,
                    Some(jumped_piece) => jumped_piece.color != piece.color
                }
            }
        }
    }

    fn valid_destination(&self, movement: Move) -> bool {
        let Coordinate(x, y) = movement.to;
        self.board[x][y].is_none()
    }

    /// Updates the valid moves list for a given piece and location.
    ///
    /// Calculates all legal moves for the piece and location, then updates
    /// the global list for the piece color.
    fn update_valid_moves_for(&mut self, location: Coordinate, piece: GamePiece) {
        let color = piece.color;
        let mut new_moves = self.valid_moves_for(location);
        let mut legal_new_moves = self.legal_moves(piece, new_moves);

        self.valid_moves
            .entry(color)
            .and_modify(|color_list| color_list.append(&mut legal_new_moves))
            .or_insert(legal_new_moves);
    }

    /// Removes moves for a given piece and location from the global list
    fn remove_valid_moves_for(&mut self, location: Coordinate, piece: GamePiece) {
        let color_list = if let Some(list) = self.valid_moves.get(&piece.color) {
            list
        } else {
            return;
        };

        let new_moves: Vec<Move> = color_list
            .iter()
            .cloned()
            .filter(|movement| movement.from.eq(&location))
            .collect();

        self.valid_moves.insert(piece.color, new_moves);
    }

    /// Removes old moves for a location, and adds the updated moves.
    ///
    /// If none exist, simply inserts the new moves.
    fn update_valid_moves(&mut self, location: Coordinate, piece: GamePiece) {
        let Coordinate(x, y) = location;

        match self.board[x][y] {
            None => self.remove_valid_moves_for(location, piece),
            Some(piece) => {
                self.remove_valid_moves_for(location, piece);
                self.update_valid_moves_for(location, piece);
            }
        }
    }

    /// Removes a piece from a given location on the board.
    fn remove_piece(&mut self, location: Coordinate, piece: GamePiece) {
        let Coordinate(x, y) = location;
        self.board[x][y] = None;
        self.update_valid_moves(location, piece)
    }

    /// Sets a piece on a given location on the board.
    fn set_piece(&mut self, location: Coordinate, piece: GamePiece) {
        let Coordinate(x, y) = location;
        self.board[x][y] = Some(piece);
        self.update_valid_moves(location, piece)
    }

    /// Returns the coordinates of a piece that has been jumped during a given
    /// move.
    fn jumped_piece_coords(&self, from: Coordinate, to: Coordinate) -> Option<Coordinate> {
        let Coordinate(from_x, from_y) = from;
        let Coordinate(to_x, to_y) = to;

        let x_delta: i8 = to_x as i8 - from_x as i8;
        let y_delta: i8 = to_y as i8 - from_y as i8;

        if x_delta < 2 && y_delta < 2 {
            None
        } else {
            let piece_x = (from_x as i8 + x_delta / 2) as usize;
            let piece_y = (from_y as i8 + y_delta / 2) as usize;

            self.board[piece_x][piece_y].map(|_game_piece| Coordinate(piece_x, piece_y))
        }
    }

    /// Returns whether or not a given piece should be crowned.
    fn should_crown(&self, piece: GamePiece, location: Coordinate) -> bool {
        const WHITE_ROW: usize = START_INDEX;
        const BLACK_ROW: usize = END_INDEX;
        let Coordinate(_x, y) = location;

        match piece.color {
            PieceColor::Black => y == WHITE_ROW,
            PieceColor::White => y == BLACK_ROW,
        }
    }

    /// Mutates crowned state of the piece at a given location to be true.
    fn crown(&mut self, location: Coordinate) -> bool {
        let Coordinate(x, y) = location;

        if let Some(piece) = self.board[x][y] {
            self.board[x][y] = Some(GamePiece::crown(piece));
            true
        } else {
            false
        }
    }

    /// Returns whether a piece at the given location has been crowned.
    fn is_crowned(&mut self, location: Coordinate) -> bool {
        let Coordinate(x, y) = location;

        if let Some(piece) = self.board[x][y] {
            piece.crowned
        } else {
            false
        }
    }

    /// Advances the turn to the next player.
    fn advance_turn(&mut self) {
        match self.current_turn {
            PieceColor::Black => self.current_turn = PieceColor::White,
            PieceColor::White => self.current_turn = PieceColor::Black,
        }

        self.move_count += 1
    }
}

#[cfg(test)]
mod test {
    use super::super::board::{Coordinate, GamePiece, Move, PieceColor};
    use super::GameEngine;

    #[test]
    fn should_crown() {
        let engine = GameEngine::new();
        let black = GamePiece::new(PieceColor::Black);
        let res = engine.should_crown(black, Coordinate(3, 0));
        assert!(res);
        let res_nocrown = engine.should_crown(black, Coordinate(5, 2));
        assert_eq!(res_nocrown, false);
    }

    #[test]
    fn mut_crown() {
        let mut engine = GameEngine::new();
        engine.initialize();
        let crowned = engine.crown(Coordinate(1, 0));
        assert!(crowned);
        assert!(engine.is_crowned(Coordinate(1, 0)));
    }

    #[test]
    fn advance_turn() {
        let mut engine = GameEngine::new();
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::White);
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::Black);
        assert_eq!(engine.move_count(), 2);
    }

    #[test]
    fn move_targets() {
        let c1 = Coordinate(0, 5);
        let targets = c1.valid_moves().collect::<Vec<Coordinate>>().sort();
        assert_eq!(targets, [Coordinate(1, 6), Coordinate(1, 4)].sort());

        let c2 = Coordinate(1, 6);
        let targets2 = c2.valid_moves().collect::<Vec<Coordinate>>().sort();
        assert_eq!(
            targets2,
            [
                Coordinate(0, 7),
                Coordinate(2, 7),
                Coordinate(2, 5),
                Coordinate(0, 5)
            ].sort()
        );

        let c3 = Coordinate(2, 5);
        let targets3 = c3.valid_moves().collect::<Vec<Coordinate>>().sort();
        assert_eq!(
            targets3,
            [
                Coordinate(1, 6),
                Coordinate(3, 6),
                Coordinate(3, 4),
                Coordinate(1, 4)
            ].sort()
        );
    }

    #[test]
    fn valid_moves() {
        let coordinate_1 = Coordinate(0, 5);
        let coordinate_2 = Coordinate(2, 5);
        let destination_1 = Coordinate(1, 4);
        let destination_2 = Coordinate(3, 4);

        let mut engine = GameEngine::new();
        engine.initialize();
        let valid_move_list = engine.valid_moves.get(&PieceColor::Black).unwrap();

        let movement_1 = Move {
            from: coordinate_1,
            to: destination_1,
        };

        let movement_2 = Move {
            from: coordinate_2,
            to: destination_1,
        };

        let movement_3 = Move {
            from: coordinate_2,
            to: destination_2,
        };

        assert!(valid_move_list.contains(&movement_1));
        assert!(valid_move_list.contains(&movement_2));
        assert!(valid_move_list.contains(&movement_3));
    }

    #[test]
    fn legal_moves_black() {
        let mut engine = GameEngine::new();
        engine.initialize();
        let moves = engine.valid_moves.get_mut(&PieceColor::Black).unwrap().sort();
        let expected_moves = Vec::from([
            Move {
                from: Coordinate(0, 5),
                to: Coordinate(1, 4),
            },
            Move {
                from: Coordinate(2, 5),
                to: Coordinate(3, 4),
            },
            Move {
                from: Coordinate(2, 5),
                to: Coordinate(1, 4),
            },
            Move {
                from: Coordinate(4, 5),
                to: Coordinate(5, 4),
            },
            Move {
                from: Coordinate(4, 5),
                to: Coordinate(3, 4),
            },
            Move {
                from: Coordinate(6, 5),
                to: Coordinate(7, 4),
            },
            Move {
                from: Coordinate(6, 5),
                to: Coordinate(5, 4),
            }
        ]).sort();

        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn legal_moves_white() {
        let mut engine = GameEngine::new();
        engine.initialize();
        engine.advance_turn();
        let moves = engine.valid_moves.get_mut(&PieceColor::White).unwrap().sort();
        let expected_moves = Vec::from([
            Move {
                from: Coordinate(1, 2),
                to: Coordinate(0, 3),
            },
            Move {
                from: Coordinate(1, 2),
                to: Coordinate(2, 3),
            },
            Move {
                from: Coordinate(3, 2),
                to: Coordinate(2, 3),
            },
            Move {
                from: Coordinate(3, 2),
                to: Coordinate(4, 3),
            },
            Move {
                from: Coordinate(5, 2),
                to: Coordinate(4, 3),
            },
            Move {
                from: Coordinate(5, 2),
                to: Coordinate(6, 3),
            },
            Move {
                from: Coordinate(7, 2),
                to: Coordinate(6, 3),
            }
        ]).sort();
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn jumps_validation() {
        let mut engine = GameEngine::new();
        engine.initialize();
        engine.board[1][4] = Some(GamePiece::new(PieceColor::White)); // this should be jumpable from 0,5 to 2,3
        let moves = engine.valid_moves.get_mut(&PieceColor::Black).unwrap().sort();
        let expected_moves = Vec::from([
            Move {
                from: Coordinate(0, 5),
                to: Coordinate(2, 3),
            },
            Move {
                from: Coordinate(2, 5),
                to: Coordinate(0, 3)
            },
            Move {
                from: Coordinate(2, 5),
                to: Coordinate(3, 4)
            },
            Move {
                from: Coordinate(4, 5),
                to: Coordinate(5, 4)
            },
            Move {
                from: Coordinate(4, 5),
                to: Coordinate(3, 4)
            },
            Move {
                from: Coordinate(6, 5),
                to: Coordinate(7, 4)
            },
            Move {
                from: Coordinate(6, 5),
                to: Coordinate(5, 4)
            }
        ]).sort();
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn valid_jumps() {
        let c1 = Coordinate(3, 3);
        let targets = c1.valid_jumps().collect::<Vec<Coordinate>>().sort();
        assert_eq!(
            targets,
            [
                Coordinate(5, 1),
                Coordinate(5, 5),
                Coordinate(1, 1),
                Coordinate(1, 5)
            ].sort()
        );
    }

    #[test]
    fn test_basic_move() {
        let mut engine = GameEngine::new();
        engine.initialize();
        let res = engine.move_piece(&Move::new((0, 5), (1, 4)));
        assert!(res.is_ok());

        let old = engine.board[0][5];
        let new = engine.board[1][4];
        assert_eq!(old, None);
        assert_eq!(
            new,
            Some(GamePiece {
                color: PieceColor::Black,
                crowned: false
            })
        );

        // fail to perform illegal move
        let res = engine.move_piece(&Move::new((1, 4), (2, 4))); // can't move horiz
        assert!(!res.is_ok());
        assert_eq!(engine.board[2][4], None);
    }
}
