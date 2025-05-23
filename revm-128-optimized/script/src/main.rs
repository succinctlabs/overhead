use sp1_sdk::{utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("/Users/ksk/Desktop/Succinct/overhead/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/revm-128-optimized-program");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '100000' to it.
    let n = 100000u128;

    // The input stream that the program will read from using `sp1_zkvm::io::read`. Note that the
    // types of the elements in the input stream must match the types being read in the program.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Create a `ProverClient` method.
    let client = ProverClient::new();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    // Generate the proof for the given program and input.
    // let (pk, vk) = client.setup(ELF);
    // let mut proof = client.prove(&pk, stdin).run().unwrap();

    // println!("generated proof");

    // // Read and verify the output.
    // //
    // // Note that this output is read from values commited to in the program using
    // // `sp1_zkvm::io::commit`.
    // let _ = proof.public_values.read::<u32>();
    // let a = proof.public_values.read::<Vec<u8>>();

    // println!("a: {:?}", a);

    // // Verify proof and public values
    // client.verify(&proof, &vk).expect("verification failed");

    // // Test a round trip of proof serialization and deserialization.
    // proof
    //     .save("proof-with-pis.bin")
    //     .expect("saving proof failed");
    // let deserialized_proof =
    //     SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // // Verify the deserialized proof.
    // client
    //     .verify(&deserialized_proof, &vk)
    //     .expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}