#![no_main]
sp1_zkvm::entrypoint!(main);

use std::u128;

use revm::primitives::Bytes;
use revm::primitives::{Bytecode, CancunSpec};
use revm_interpreter::analysis::to_analysed;
use revm_interpreter::opcode::InstructionTable;
use revm_interpreter::DummyHost;
use revm_interpreter::{Contract, Interpreter, EMPTY_SHARED_MEMORY};

/// The bytecode we want to execute inside the EVM.
/// This should be compiled from a contract with `function fib(uint128)`
const BYTECODE_STR: &str = "608060405234801561000f575f80fd5b5060043610610029575f3560e01c8063c6c2ea171461002d575b5f80fd5b610047600480360381019061004291906100ff565b61005d565b6040516100549190610154565b60405180910390f35b5f80820361006d575f90506100c3565b5f600190505f600190505f600290505b848110156100bc575f6f7fffffffffffffffffffffffffffffff806100a5576100a461016d565b5b83850890508293508092505080600101905061007d565b5080925050505b919050565b5f80fd5b5f819050919050565b6100de816100cc565b81146100e8575f80fd5b50565b5f813590506100f9816100d5565b92915050565b5f60208284031215610114576101136100c8565b5b5f610121848285016100eb565b91505092915050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b61014e8161012a565b82525050565b5f6020820190506101675f830184610145565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffdfea2646970667358221220b7f1d5e99c05d0958c32b31c216223c270a4b3693da72707fabbda6bc6c2eceb64736f6c634300081a0033";

pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u128>();
    sp1_zkvm::io::commit(&n);

    // First, we need to format the call data.
    // 
    // The call data starts with the function selector.
    let mut call_data_raw = hex::decode("c6c2ea17").unwrap();
    // Then, we append the padded value of n
    let mut padded_bytes = [0u8; 32];
    padded_bytes[16..32].copy_from_slice(&n.to_be_bytes()); 
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
        u128::MAX as u64,
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

    // Decode result as u128.
    let out: Vec<u8> = raw_out.into_result_return().unwrap().output.into();
    let result = u128::from_be_bytes(out[16..32].try_into().unwrap());

    // Commit to the output.
    sp1_zkvm::io::commit(&result);
}
