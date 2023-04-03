(module
  (; Imports ;)
  (import "events" "piece_moved"
    (func $notify_piece_moved (param $fromX i32) (param $fromY i32)
                              (param $toX i32) (param $toY i32)
    )
  )
  (import "events" "piece_crowned"
    (func $notify_piece_crowned (param $x i32) (param $y i32))
  )

  (memory $mem 1)

  (; Globals ;)
  ;; Board boundary indexes
  (global $START_INDEX i32 (i32.const 0))
  (global $END_INDEX i32 (i32.const 7))

  ;; Space/Piece values
  (global $EMPTY i32 (i32.const 0))
  (global $BLACK i32 (i32.const 1))
  (global $WHITE i32 (i32.const 2))
  (global $CROWN i32 (i32.const 4))

  ;; For masking operations only:
  (global $NOCROWN i32 (i32.const 3))

  ;; Track the current turn
  (global $currentTurn (mut i32) (i32.const 0))

  (; Board Functions ;)
  ;; Get the stack index for a given position
  (func $indexForPosition (param $x i32) (param $y i32) (result i32)
    (i32.add
      (i32.mul
        (i32.const 8)
        (local.get $y)
      )
      (local.get $x)
    )
  )

  ;; Byte Offset = ( x + y * 8 ) * 4
  (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
    (i32.mul
      (call $indexForPosition (local.get $x) (local.get $y))
      (i32.const 4)
    )
  )

  (;
    ________________________________________________________
   | Bitwise Masking Operations                             |
   |========================================================|
   | Operation | Wasm Function | Bitmask Action             |
   |-----------|---------------|----------------------------|
   | AND       | i32.and       | Query the bit value        |
   | OR        | i32.or        | Sets the bit to true (1)   |
   | XOR       | i32.xor       | Toggles the value of a bit |
    --------------------------------------------------------
  ;)

  (;
    ________________________________________________________________
   | Checkerboard Square States                                     |
   |================================================================|
   | Binary Value (last 8 bits)  | Decimal Value | Game Meaning     |
   |-----------------------------|---------------|------------------|
   | [24 unused bits]...00000000 |       0       | Unoccupied Space |
   | [24 unused bits]...00000001 |       1       | Black Piece      |
   | [24 unused bits]...00000010 |       2       | White Piece      |
   | [24 unused bits]...00000100 |       4       | Crowned Piece    |
    ----------------------------------------------------------------
    (Crowned black = 5 / 00000101, Crowned white = 6 / 00000110)
  ;)

  ;; Determine if a piece has been crowned:
  (func $isCrowned (param $piece i32) (result i32)
    (i32.eq
      (i32.and (local.get $piece) (global.get $CROWN))
      (global.get $CROWN)
    )
  )

  ;; Determine if a piece is white:
  (func $isWhite (param $piece i32) (result i32)
    (i32.eq
      (i32.and (local.get $piece) (global.get $WHITE))
      (global.get $WHITE)
    )
  )

  ;; Determine if the piece is black
  (func $isBlack (param $piece i32) (result i32)
    (i32.eq
      (i32.and (local.get $piece) (global.get $BLACK))
      (global.get $BLACK)
    )
  )

  ;; Crown a piece (no mutation, just returns the new value):
  (func $crowned (param $piece i32) (result i32)
    (i32.or (local.get $piece) (global.get $CROWN))
  )

  (;
    ----------------------------------------------------------------------
   | UnCrowned Bitmask Examples                                           |
   |======================================================================|
   | Value    | Meaning         | Operation | Mask     | Result           |
   |----------------------------------------------------------------------|
   | 0101 (5) | Crowned Black   | i32.and   | 0011 (3) | 0001 (1 / Black) |
   | 0001 (1) | Uncrowned Black | i32.and   | 0011 (3) | 0001 (1 / Black) |
   | 0110 (6) | Crowned White   | i32.and   | 0011 (3) | 0010 (2 / White) |
    ----------------------------------------------------------------------
  ;)

  ;; Remove a crown (no mutation, just returns the new value):
  (func $unCrowned (param $piece i32) (result i32)
    (i32.and (local.get $piece) (global.get $NOCROWN))
  )

  ;; Sets a piece on the board
  (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
    (i32.store
      (call $offsetForPosition
        (local.get $x)
        (local.get $y)
      )
      (local.get $piece)
    )
  )

  ;; Mark a space empty
  (func $setEmpty (param $x i32) (param $y i32)
    (call $setPiece (local.get $x) (local.get $y) (global.get $EMPTY))
  )

  ;; Detect if a value is within a range of values (inclusive of high and low)
  (func $inRange (param $low i32) (param $high i32) (param $value i32)
                 (result i32)
    (i32.and
      (i32.ge_s (local.get $value) (local.get $low))
      (i32.le_s (local.get $value) (local.get $high))
    )
  )

  ;; Gets a piece from the board. Out of range error causes a trap
  (func $getPiece (param $x i32) (param $y i32) (result i32)
    (if (result i32)
      (block (result i32)
        ;; validate x and y are both in range:
        (i32.and
          (call $inRange
            (global.get $START_INDEX)
            (global.get $END_INDEX)
            (local.get $x)
          )
          (call $inRange
            (global.get $START_INDEX)
            (global.get $END_INDEX)
            (local.get $y)
          )
        )
      )
      ;; Load the piece (or "empty" value) at that position:
      (then
        (i32.load
          (call $offsetForPosition (local.get $x) (local.get $y))
        )
      )
      ;; Otherwise, abort
      (else
        (unreachable)
      )
    )
  )

  ;; Get the owner of the current turn:
  (func $getTurnOwner (result i32)
    (global.get $currentTurn)
  )

  ;; Set the turn owner
  (func $setTurnOwner (param $player i32)
    (global.set $currentTurn (local.get $player))
  )

  ;; Determine if a given player is the turn owner
  (func $playerIsTurnOwner (param $player i32) (result i32)
    (i32.gt_s
      (i32.and (local.get $player) (call $getTurnOwner))
      (i32.const 0)
    )
  )

  ;; Swap the turn owner at the end of a turn
  (func $toggleTurnOwner
    (if (i32.eq (call $getTurnOwner) (global.get $WHITE))
      (then (call $setTurnOwner (global.get $BLACK)))
      (else (call $setTurnOwner (global.get $WHITE)))
    )
  )

  ;; Convert a piece into a crowned piece, and invokes a host notification
  (func $crownPiece (param $x i32) (param $y i32)
    (local $piece i32)
    (local.set $piece (call $getPiece (local.get $x) (local.get $y)))

    (call $setPiece (local.get $x) (local.get $y)
                    (call $crowned (local.get $piece))
    )

    ;; This will be implemented by our host program
    (call $notify_piece_crowned (local.get $x) (local.get $y))
  )

  ;; Determine if a piece is eligible to be crowned
  ;; (for black: row 0, for white: row 7)
  (func $canCrown (param $y i32) (param $piece i32) (result i32)
    (i32.or
      (i32.and
        (i32.eq (local.get $y) (global.get $START_INDEX))
        (call $isBlack (local.get $piece))
      )
      (i32.and
        (i32.eq (local.get $y) (global.get $END_INDEX))
        (call $isWhite (local.get $piece))
      )
    )
  )

  ;; get the distance between two squares
  (func $getDistance (param $a i32) (param $b i32) (result i32)
    (i32.sub (local.get $a) (local.get $b))
  )

  ;; Ensure jump is a valid distance (1 or 2 squares, in the case of a jump)
  (func $validMoveDistance (param $from i32) (param $to i32) (result i32)
    (local $distance i32)
    (local.set $distance
      (if (result i32)
        (i32.gt_s (local.get $to) (local.get $from))
        (then
          (call $getDistance (local.get $to) (local.get $from))
        )
        (else
          (call $getDistance (local.get $from) (local.get $to))
        )
      )
    )
    (i32.le_u (local.get $distance) (i32.const 2))
  )

  ;; Determine if a move is valid
  (func $isValidMove (param $fromX i32) (param $fromY i32)
                   (param $toX i32) (param $toY i32) (result i32)
    (local $player i32)
    (local $target i32)

    (local.set $player (call $getPiece (local.get $fromX) (local.get $fromY)))
    (local.set $target (call $getPiece (local.get $toX) (local.get $toY)))

    (if (result i32)
      (block (result i32)
        (i32.and
          ;; valid distance (if y is valid, x will be as well)
          (call $validMoveDistance (local.get $fromY) (local.get $toY))
          (i32.and
            ;; valid turn (not trying to move a piece of the opposite color)
            (call $playerIsTurnOwner (local.get $player))
            ;; valid target (no piece exists)
            (i32.eq (local.get $target) (global.get $EMPTY))
          )
        )
      )
      (then (i32.const 1))
      (else (i32.const 0))
    )
  )

  ;; Exportable move function for the game host:
  (func $move (param $fromX i32) (param $fromY i32)
              (param $toX i32) (param $toY i32) (result i32)
    (if (result i32)
      (block (result i32)
        (call $isValidMove (local.get $fromX) (local.get $fromY)
                           (local.get $toX) (local.get $toY)
        )
      )
      (then
        (call $make_move (local.get $fromX) (local.get $fromY)
                         (local.get $toX) (local.get $toY)
        )
      )
      (else (i32.const 0))
    )
  )

  ;; Internal move function, performs actual move post-validation of the target.
  ;; TODO:
  ;;   - Removing jumped pieces
  ;;   - Detecting victory
  (func $make_move (param $fromX i32) (param $fromY i32)
              (param $toX i32) (param $toY i32) (result i32)

    ;; Fetch the piece we're moving given the coordinates:
    (local $piece i32)
    (local.set $piece (call $getPiece (local.get $fromX) (local.get $fromY)))

    ;; Set the new coordinates to contain our piece, and wipe the previous
    (call $setPiece (local.get $toX) (local.get $toY) (local.get $piece))
    (call $setEmpty (local.get $fromX) (local.get $fromY))

    ;; Emit a notification
    (call $notify_piece_moved (local.get $fromX) (local.get $fromY)
                              (local.get $toX) (local.get $toY)
    )

    ;; Crown the piece, if warranted
    (if (call $canCrown (local.get $toY) (local.get $piece))
      (then (call $crownPiece (local.get $toX) (local.get $toY)))
    )

    ;; Toggle the turn owner
    (call $toggleTurnOwner)

    ;; Return a truthy value
    (i32.const 1)
  )

  (; Initialization ;)
  ;; Manually place each piece
  (func $initBoard
    (local $whiteRowOne i32)
    (local $whiteRowTwo i32)
    (local $whiteRowThree i32)
    (local $blackRowOne i32)
    (local $blackRowTwo i32)
    (local $blackRowThree i32)

    (local.set $whiteRowOne (global.get $START_INDEX))
    (local.set $whiteRowTwo (i32.add (local.get $whiteRowOne) (i32.const 1)))
    (local.set $whiteRowThree (i32.add (local.get $whiteRowTwo) (i32.const 1)))
    (local.set $blackRowOne (global.get $END_INDEX))
    (local.set $blackRowTwo (i32.sub (local.get $blackRowOne) (i32.const 1)))
    (local.set $blackRowThree (i32.sub (local.get $blackRowTwo) (i32.const 1)))

    ;; White pieces at the top:
    ;; First Row
    (call $setPiece (i32.const 1) (local.get $whiteRowOne) (global.get $WHITE))
    (call $setPiece (i32.const 3) (local.get $whiteRowOne) (global.get $WHITE))
    (call $setPiece (i32.const 5) (local.get $whiteRowOne) (global.get $WHITE))
    (call $setPiece (i32.const 7) (local.get $whiteRowOne) (global.get $WHITE))

    ;; Second Row
    (call $setPiece (i32.const 0) (local.get $whiteRowTwo) (global.get $WHITE))
    (call $setPiece (i32.const 2) (local.get $whiteRowTwo) (global.get $WHITE))
    (call $setPiece (i32.const 4) (local.get $whiteRowTwo) (global.get $WHITE))
    (call $setPiece (i32.const 6) (local.get $whiteRowTwo) (global.get $WHITE))

    ;; Third Row
    (call $setPiece (i32.const 0) (local.get $whiteRowTwo) (global.get $WHITE))
    (call $setPiece (i32.const 2) (local.get $whiteRowTwo) (global.get $WHITE))
    (call $setPiece (i32.const 4) (local.get $whiteRowTwo) (global.get $WHITE))
    (call $setPiece (i32.const 6) (local.get $whiteRowTwo) (global.get $WHITE))

    ;; Black pieces at the bottom:
    ;; First row
    (call $setPiece (i32.const 0) (local.get $blackRowOne) (global.get $BLACK))
    (call $setPiece (i32.const 2) (local.get $blackRowOne) (global.get $BLACK))
    (call $setPiece (i32.const 4) (local.get $blackRowOne) (global.get $BLACK))
    (call $setPiece (i32.const 6) (local.get $blackRowOne) (global.get $BLACK))

    ;; Second row
    (call $setPiece (i32.const 1) (local.get $blackRowTwo) (global.get $BLACK))
    (call $setPiece (i32.const 3) (local.get $blackRowTwo) (global.get $BLACK))
    (call $setPiece (i32.const 5) (local.get $blackRowTwo) (global.get $BLACK))
    (call $setPiece (i32.const 7) (local.get $blackRowTwo) (global.get $BLACK))

    ;; Third row
    (call $setPiece (i32.const 0) (local.get $blackRowThree) (global.get $BLACK))
    (call $setPiece (i32.const 2) (local.get $blackRowThree) (global.get $BLACK))
    (call $setPiece (i32.const 4) (local.get $blackRowThree) (global.get $BLACK))
    (call $setPiece (i32.const 6) (local.get $blackRowThree) (global.get $BLACK))

    ;; Start the turn with Black
    (call $setTurnOwner (global.get $BLACK))
  )

  (; Exports ;)
  (export "getPiece" (func $getPiece))
  (export "isCrowned" (func $isCrowned))
  (export "initBoard" (func $initBoard))
  (export "getTurnOwner" (func $getTurnOwner))
  (export "move" (func $move))
  (export "memory" (memory $mem))
)
