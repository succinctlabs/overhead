#![no_main]
sp1_zkvm::entrypoint!(main);
use std::io::stdout;

use wain_exec::{DefaultImporter, Runtime, Value};
use wain_syntax_binary::parse;

// This WASM was built using wat2wasm (https://webassembly.github.io/wabt/demo/wat2wasm/)
// The original .wat file is available at ../../../fib.wat
const WASM: &[u8] = include_bytes!("../../../fib-32.wasm");
pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<i32>();
    sp1_zkvm::io::commit(&n);

    // Parse the WASM into an AST.
    let tree = match parse(WASM) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("Could not parse: {}", err);
            panic!();
        }
    };
    println!("cycle-tracker-end: set up input");

    // Set up the runtime.
    println!("cycle-tracker-start: set up runtime");
    let input = "some dummy text";
    let stdout = stdout();
    let importer = DefaultImporter::with_stdio(input.as_bytes(), stdout.lock());
    // Instantiate the module.
    let mut runtime = match Runtime::instantiate(&tree.module, importer) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("could not instantiate module: {}", err);
            panic!();
        }
    };
    println!("cycle-tracker-end: set up runtime");

    // Find and run the exported function.
    println!("cycle-tracker-start: run interpreter");
    let x = match runtime.invoke("fib", &[Value::I32(n)]) {
        Ok(ret) => {
            if let Some(Value::I32(i)) = ret {
                i
            } else {
                panic!();
            }
        }

        Err(trap) => {
            eprintln!("Function execution failed: {}", trap);
            panic!();
        }
    };

    // Commit the result.
    sp1_zkvm::io::commit(&x);
    println!("cycle-tracker-end: run interpreter");
}
