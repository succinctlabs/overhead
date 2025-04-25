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
const BYTECODE_STR: &str = "608060405234801561000f575f5ffd5b5060043610610029575f3560e01c8063923998ab1461002d575b5f5ffd5b61004760048036038101906100429190610149565b61005d565b6040516100549190610183565b60405180910390f35b5f5f826fffffffffffffffffffffffffffffffff160361007f575f90506100fb565b5f60019050600191505f600290505b836fffffffffffffffffffffffffffffffff16816fffffffffffffffffffffffffffffffff1610156100f8575f6f7fffffffffffffffffffffffffffffff84846100d891906101c9565b6100e29190610239565b905083925080935050808060010191505061008e565b50505b919050565b5f5ffd5b5f6fffffffffffffffffffffffffffffffff82169050919050565b61012881610104565b8114610132575f5ffd5b50565b5f813590506101438161011f565b92915050565b5f6020828403121561015e5761015d610100565b5b5f61016b84828501610135565b91505092915050565b61017d81610104565b82525050565b5f6020820190506101965f830184610174565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6101d382610104565b91506101de83610104565b925082820190506fffffffffffffffffffffffffffffffff8111156102065761020561019c565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61024382610104565b915061024e83610104565b92508261025e5761025d61020c565b5b82820690509291505056fea2646970667358221220d4b542f2594fc62381ab74f670e1c8cd019d7ae383f15787e8ccabe95d479eac64736f6c634300081d0033";

pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u128>();
    sp1_zkvm::io::commit(&n);

    // First, we need to format the call data.
    // 
    // The call data starts with the function selector.
    let mut call_data_raw = hex::decode("923998ab").unwrap();

    // Then, we append the padded value of n
    let mut padded_bytes = [0u8; 32];
    padded_bytes[16..32].copy_from_slice(&n.to_be_bytes()); // pad u128 to 32 bytes
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
