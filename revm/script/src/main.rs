use sp1_sdk::utils::setup_logger;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("revm-program");

fn main() {
    // Setup logging.
    setup_logger();

    // Create an input stream and write '1000' to it.
    let n = 1000u32;

    // The input stream that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Create a `ProverClient` method.
    let client = ProverClient::from_env();

    // Execute the program.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("Total cycles: {}", report.total_instruction_count());
}
