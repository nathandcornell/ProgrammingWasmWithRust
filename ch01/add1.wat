(; first way (fewer parens):
  (module
    (func $add (param $lhs i32) (param $rhs i32) (result i32)
      local.get $lhs
      local.get $rhs
      i32.add)
    (export "add" (func $add))
  )
;)

(; second way (more parens, but more human-readable) ;)

(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    (i32.add
      (local.get $lhs)
      (local.get $rhs)
    )
  )
  (export "add" (func $add))
)
