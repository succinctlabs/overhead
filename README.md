# Wasm / EVM interpreters in SP1

These experiments demonstrate that running a WASM interpreter in SP1 is feasible. We write a simple program that computes the n'th fibonacci number mod 7919, and compile it to a .wat and evm bytecode. As a baseline, we implement this program in plain rust. We run the evm bytecode through `revm-interpreter`, and we run the .wat through the `wasmi` and `wain` interpreters. 

We evaluate the number of sp1 cycles it takes to execute the fibonacci program in various interpreters.

## Running

For the experiment you want to run, go to the `[experiment]/script` directory and run `RUST_LOG=info cargo run --release`. For example, if you run `wain-fibonacci`, you might get a result like this.

```
2024-08-28T18:51:58.308299Z  INFO execute: clk = 0 pc = 0x20aee4    
2024-08-28T18:51:58.308405Z  INFO execute: ┌╴set up input    
2024-08-28T18:51:58.308887Z  INFO execute: └╴12,813 cycles    
2024-08-28T18:51:58.308903Z  INFO execute: ┌╴set up runtime    
2024-08-28T18:51:58.308952Z  INFO execute: └╴1,025 cycles    
2024-08-28T18:51:58.308970Z  INFO execute: ┌╴run interpreter    
2024-08-28T18:51:58.373942Z  INFO execute: └╴2,520,476 cycles  
...
```

The setup costs are roughly fixed, and the "run interpreter" part is responsible for executing the actual instructions of the wasm/evm program.
## Results

| Experiment Name | Set up input | Set up runtime | Interpreter loop | Total cycles |
|-----------------|--------------|----------------|------------------|--------------|
| Baseline        | --           | --             | --               | 16,944       |
| Wasmi           | 43,771       | 3,997          | 183,348          | 237,382      |
| Wain            | 12,813       | 1,025          | 2,520,476        | 2,540,679    |
| Revm            | 42,395       | 11,262         | 9,095,092        | 9,153,940    |

## Other notes

`fib.sol` compiles into the EVM bytecode I used for `revm`. 

The wasm program executes in 5,998 instructions.

The evm program executes in 151,903 instructions.

