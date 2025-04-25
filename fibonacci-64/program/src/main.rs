//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Use u64 for larger numbers
    let n = sp1_zkvm::io::read::<u64>();
    sp1_zkvm::io::commit(&n);

    // Use u64 for fibonacci calculation
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 65776547668456965;  // Modulus to prevent overflow
        a = b;
        b = c;
    }

    sp1_zkvm::io::commit(&a);
}
