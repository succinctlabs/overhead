(module
  (func $fib (param $n i64) (result i64)
    (local $a i64)
    (local $b i64)
    (local $temp i64)

    (if (i64.le_s (local.get $n) (i64.const 1))
      (then
        (local.get $n)
        (return)
      )
    )

    (local.set $a (i64.const 0))
    (local.set $b (i64.const 1))

    (loop $loop
      (local.set $temp (local.get $b))
      (local.set $b 
        (i64.rem_u 
          (i64.add (local.get $a) (local.get $b)) 
          (i64.const 65776547668456965)
        )
      )
      (local.set $a (local.get $temp))
      (local.set $n (i64.sub (local.get $n) (i64.const 1)))
      (br_if $loop (i64.gt_s (local.get $n) (i64.const 1)))
    )

    (local.get $b)
  )

  (export "fib" (func $fib))
)