use sp1_sdk::{ProverClient, SP1Stdin};
use sp1_sdk::utils::setup_logger;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup logging.
    setup_logger();

    // Generate proof.
    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    stdin.write(&10000);

    // Execute the program.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("Total cycles: {}", report.total_instruction_count());
}
