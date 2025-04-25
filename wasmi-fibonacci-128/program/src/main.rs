#![no_main]
sp1_zkvm::entrypoint!(main);
use wasmi::*;

/// This WASM was built using wat2wasm (https://webassembly.github.io/wabt/demo/wat2wasm/)
/// The original .wat file is available at ../../../fib.wat
const WASM: &[u8] = include_bytes!("../../../fib-128.wasm");

pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<i64>();
    sp1_zkvm::io::commit(&n);
    
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    
    // Set up the module by parsing the Wasm binary.
    let module = Module::new(&engine, &mut &WASM[..]).unwrap();
    println!("cycle-tracker-end: set up input");
    
    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which we don't need in this case.
    println!("cycle-tracker-start: set up runtime");
    let mut store = Store::new(&engine, ());

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let linker = <Linker<()>>::new(&engine);
    
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    let instance = linker
        .instantiate(&mut store, &module).unwrap()
        .start(&mut store).unwrap();
    let fib = instance.get_typed_func::<i64, (i64, i64)>(&store, "fib").unwrap();
    println!("cycle-tracker-end: set up runtime");
    
    // And finally we can call the wasm!
    println!("cycle-tracker-start: run interpreter");
    let (high, low) = fib.call(&mut store, n).unwrap();

    // Commit the result.
    sp1_zkvm::io::commit(&(high, low));
    println!("cycle-tracker-end: run interpreter");
}
