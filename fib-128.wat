  (module
    ;; Import the memory to store our results
    ;; (memory 1)
    ;; (export "memory" (memory 0))
  
    ;; Import the 'print' function for outputting the result
    ;; (import "env" "print" (func $print (param i32)))
  
    ;; Function to calculate the n-th Fibonacci number
      (func $fib (param $n i32) (result i32)
      (local $a i32)
      (local $b i32)
      (local $temp i32)
  
      ;; If n <= 1, return n
      (if (i32.le_s (local.get $n) (i32.const 1))
        (then
          (local.get $n)
          (return)
        )
      )
  
      ;; Set initial values for the loop
      (local.set $a (i32.const 0))
      (local.set $b (i32.const 1))
  
      ;; Loop from 2 to n
      (loop $loop
        (local.set $temp (local.get $b))
            (local.set $b 
              (i32.rem_s 
                (i32.add (local.get $a) (local.get $b)) 
                (i32.const 7919)
              )
            )
      (local.set $a (local.get $temp))
        (local.set $n (i32.sub (local.get $n) (i32.const 1)))
        (br_if $loop (i32.gt_s (local.get $n) (i32.const 1)))
      )
  
      ;; Return the result
      (local.get $b)
    )
  
    ;; Export the Fibonacci function
    (export "fib" (func $fib))
  )