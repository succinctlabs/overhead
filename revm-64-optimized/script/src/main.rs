use sp1_sdk::utils::setup_logger;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/revm-64-optimized-program");

fn main() {
    setup_logger();
    println!("ELF file size: {} bytes", ELF.len());

    // Create an input stream and write '10000' to it.
    let n = 10000u64;
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
