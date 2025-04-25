use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    let n = 100000u128;  // Using u128 for the input
    stdin.write(&n);
    
    // Just execute and check cycles
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );
}