use sp1_sdk::utils::setup_logger;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/revm-128-program");

fn main() {
    setup_logger();
    println!("UNOPTIMIZED PROGRAM");
    println!("ELF file size: {} bytes", ELF.len());

    // Create an input stream and write '100000' to it.
    let n = 100000u128;
    println!("Testing with input n = {}", n);

    // The input stream that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Create a `ProverClient` method.
    let client = ProverClient::from_env();

    // Execute the program.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("Total cycles: {}", report.total_instruction_count());
}
