// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use std::u64;

use revm::primitives::Bytes;
use revm::primitives::{Bytecode, CancunSpec};
use revm_interpreter::analysis::to_analysed;
use revm_interpreter::opcode::InstructionTable;
use revm_interpreter::DummyHost;
use revm_interpreter::{Contract, Interpreter, EMPTY_SHARED_MEMORY};

/// The bytecode we want to execute inside the EVM.
/// This is compiled from `../../../fib.sol` using Remix, an online Solidity compiler.
const BYTECODE_STR: &str = "608060405234801561000f575f5ffd5b5060043610610029575f3560e01c8063e78692bb1461002d575b5f5ffd5b61004760048036038101906100429190610120565b61005d565b604051610054919061015a565b60405180910390f35b5f5f8267ffffffffffffffff1603610077575f90506100da565b5f60019050600191505f600290505b8367ffffffffffffffff168167ffffffffffffffff1610156100d7575f66e9af6bee54760584846100b791906101a0565b6100c19190610208565b9050839250809350508080600101915050610086565b50505b919050565b5f5ffd5b5f67ffffffffffffffff82169050919050565b6100ff816100e3565b8114610109575f5ffd5b50565b5f8135905061011a816100f6565b92915050565b5f60208284031215610135576101346100df565b5b5f6101428482850161010c565b91505092915050565b610154816100e3565b82525050565b5f60208201905061016d5f83018461014b565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6101aa826100e3565b91506101b5836100e3565b9250828201905067ffffffffffffffff8111156101d5576101d4610173565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f610212826100e3565b915061021d836100e3565b92508261022d5761022c6101db565b5b82820690509291505056fea26469706673582212203932b539c23c4b7e1930089abd8d611451b97183b207453f653c461136cc1ac664736f6c634300081d0033";

pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u64>();
    sp1_zkvm::io::commit(&n);

    // First, we need to format the call data.
    // 
    // The call data starts with the function selector.
    let mut call_data_raw = hex::decode("e78692bb").unwrap(); // fib(uint64)

    // Then, we append the padded value of n
    let mut padded_bytes = [0u8; 32];
    padded_bytes[24..32].copy_from_slice(&n.to_be_bytes());
    call_data_raw.extend(padded_bytes);
    let input = Bytes::from(call_data_raw);

    // We also need to read the bytecode from `BYTECODE_STR`.
    let bytecode = to_analysed(
        Bytecode::new_raw_checked(Bytes::copy_from_slice(&hex::decode(BYTECODE_STR).unwrap()))
            .unwrap(),
    );
    println!("cycle-tracker-end: set up input");

    // To set up the interpreter, we first instantiate it with the input and bytecode.    println!("cycle-tracker-start: set up runtime");
    let mut interp = Interpreter::new(
        Contract {
            input,
            bytecode,
            ..Default::default()
        },
        u64::MAX,
        true,
    );

    // The Revm interpreter requires a host that stores information about the execution context.
    // Since we're only executing a pure function, we set up a dummy host.
    let mut host = DummyHost::default();

    // We get an instruction table from the Cancun Spec.
    let table: &InstructionTable<DummyHost> =
        &revm_interpreter::opcode::make_instruction_table::<DummyHost, CancunSpec>();
    println!("cycle-tracker-end: set up runtime");

    // Finally, we run the interpreter.
    println!("cycle-tracker-start: interpreter");
    let raw_out = interp.run(EMPTY_SHARED_MEMORY, table, &mut host);
    println!("cycle-tracker-end: interpreter");

    // Decode result as u64.
    let out: Vec<u8> = raw_out.into_result_return().unwrap().output.into();
    let result = u64::from_be_bytes(out[24..32].try_into().unwrap());

    // Commit to the output.
    sp1_zkvm::io::commit(&result);
}
