/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    let n: u64 = 1000;
    stdin.write(&n);
    
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("Execution successful! Used {} cycles.", report.total_cycles);
    
    let pk = client.prove_key(ELF).unwrap();
    let mut proof = client.prove(&pk, &stdin).run().unwrap();
    
    let n = proof.public_values.read::<u64>();
    let a = proof.public_values.read::<u64>();
    println!("Input n: {}", n);
    println!("Fibonacci number: {}", a);
    
    client.verify(&pk, &proof).run().unwrap();
    println!("Proof verified!");
} 