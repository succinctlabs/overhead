//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a system call which handles reading inputs
    // from the prover.
    let n = sp1_zkvm::io::read::<u128>();

    // Write n to public input.
    sp1_zkvm::io::commit(&n);

    // Use u128 for fibonacci calculation
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 170141183460469231731687303715884105727; // Modulus to prevent overflow
        a = b;
        b = c;
    }

    // Write the output of the program.
    //
    // Behind the scenes, this also compiles down to a system call which handles writing
    // outputs to the prover.
    sp1_zkvm::io::commit(&a);
}
