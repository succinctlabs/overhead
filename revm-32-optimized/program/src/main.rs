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
/// This is compiled from `../../../fib.sol` using Remix, an online solidity compiler.
const BYTECODE_STR: &str = "608060405234801561000f575f80fd5b5060043610610029575f3560e01c8063c6c2ea171461002d575b5f80fd5b610047600480360381019061004291906100f1565b61005d565b604051610054919061013a565b60405180910390f35b5f80820361006d575f90506100b5565b5f600190505f600190505f600290505b848110156100ae575f611eef8061009757610096610153565b5b83850890508293508092505080600101905061007d565b5080925050505b919050565b5f80fd5b5f819050919050565b6100d0816100be565b81146100da575f80fd5b50565b5f813590506100eb816100c7565b92915050565b5f60208284031215610106576101056100ba565b5b5f610113848285016100dd565b91505092915050565b5f63ffffffff82169050919050565b6101348161011c565b82525050565b5f60208201905061014d5f83018461012b565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffdfea26469706673582212202a5a8adeb76d503bd9ce8b5fa9bfdac5fa9b9301e783c1be2bbef467fbf9559b64736f6c634300081a0033";
pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u32>();
    sp1_zkvm::io::commit(&n);

    // First, we need to format the call data.
    //
    // The call data starts with the function selector.
    let mut call_data_raw = hex::decode("c6c2ea17").unwrap();

    // Then, we append the padded value of n
    let mut padded_bytes = [0u8; 32];
    padded_bytes[28..32].copy_from_slice(&n.to_be_bytes());
    call_data_raw.extend(padded_bytes);
    let input = Bytes::from(call_data_raw);

    // We also need to read the bytecode from `BYTECODE_STR`.
    let bytecode = to_analysed(
        Bytecode::new_raw_checked(Bytes::copy_from_slice(&hex::decode(BYTECODE_STR).unwrap()))
            .unwrap(),
    );
    println!("cycle-tracker-end: set up input");

    // To set up the interpreter, we first instantiate it with the input and bytecode.
    println!("cycle-tracker-start: set up runtime");
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
    let mut host = crate::DummyHost::default();

    // We get an instruction table from the Cancun Spec.
    let table: &InstructionTable<DummyHost> =
        &revm_interpreter::opcode::make_instruction_table::<DummyHost, CancunSpec>();
    println!("cycle-tracker-end: set up runtime");

    // Finally, we run the interpreter.
    println!("cycle-tracker-start: interpreter");
    let raw_out = interp.run(EMPTY_SHARED_MEMORY, table, &mut host);
    println!("cycle-tracker-end: interpreter");

    let out: Vec<u8> = raw_out.into_result_return().unwrap().output.into();
    // Commit to the output.
    sp1_zkvm::io::commit(&out);
}
