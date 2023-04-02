(module
  (memory $mem 1)

  ;; Globals:
  (global $BLACK i32 (i32.const 1))
  (global $WHITE i32 (i32.const 2))
  (global $CROWN i32 (i32.const 4))
  ;; For masking operations only:
  (global $NOCROWN i32 (i32.const 3))

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
   | unCrowned bitmask examples                                           |
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

  (export "offsetForPosition" (func $offsetForPosition))
  (export "isCrowned" (func $isCrowned))
  (export "isWhite" (func $isWhite))
  (export "isBlack" (func $isBlack))
  (export "crowned" (func $crowned))
  (export "unCrowned" (func $unCrowned))
)
