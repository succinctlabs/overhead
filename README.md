# Interpreter Overhead in SP1

These experiments demonstrate the overhead incurred while running WASM or EVM interpreters in SP1 over native Rust. We evaluate programs that compute the n-th Fibonacci numbers increasingly for increasingly larger n, along with varying integer arithmetic in `u32`, `u64`, and `u128`, and compile them to .wat files and evm bytecode. As a baseline, we implement these programs in native rust. We run the evm bytecode through the `revm` interpreter, and we run the .wat through the `wasmi` interpreter. 

We evaluate the number of sp1 cycles it takes to execute the Fibonacci programs for  `n=` (`1000`, `100000`, `1000000`). 

## Running

For the experiment you want to run, go to the  `[experiment]/program` directory and run `cargo prove build`. Then, go to the `[experiment]/script` directory and run `RUST_LOG=info cargo run --release`. For example, if you run `revm-32`, you might get a result like this.

```
2025-03-25T17:22:09.018395Z  INFO execute: clk = 0 pc = 0x21d39c
2025-03-25T17:22:09.018482Z  INFO execute: ┌╴set up input
2025-03-25T17:22:09.019346Z  INFO execute: └╴43,474 cycles
2025-03-25T17:22:09.019362Z  INFO execute: ┌╴set up runtime
stderr: WARNING: Using insecure random number generator.
2025-03-25T17:22:09.019646Z  INFO execute: └╴10,961 cycles
2025-03-25T17:22:09.019662Z  INFO execute: ┌╴interpreter
2025-03-25T17:22:09.234599Z  INFO execute: └╴9,018,117 cycles
2025-03-25T17:22:09.236082Z  INFO execute: gas: 11389884
2025-03-25T17:22:09.236210Z  INFO execute: close time.busy=220ms time.idle=2.33µs
Total cycles: 9080285
```

The setup costs are roughly fixed, and the "run interpreter" part is responsible for executing the actual instructions of the wasm/evm program.

## Results

| Experiment: u32 | Set up input | Set up runtime | Interpreter loop | Total cycles |
|-----------------|--------------|----------------|------------------|--------------|
| Baseline        | --           | --             | --               | 16,947       |
| Wasmi           | 33,767       | 2,909          | 307,625          | 347,000      |
| Revm            | 43,474       | 10,961         | 9,018,117        | 9,080,285    |
| Revm (optimized)| 32,531       | 10,961         | 2,398,175        | 2,449,400    |

| Experiment: u64 | Set up input | Set up runtime | Interpreter loop | Total cycles |
|-----------------|--------------|----------------|------------------|--------------|
| Baseline        | --           | --             | --               | 124,947      |
| Wasmi           | 34,050       | 2,910          | 3,873,431        | 3,913,090    |
| Revm            | 44,934       | 10,961         | 91,478,766       | 91,537,500   |
| Revm (optimized)| 32,091       | 10,961         | 23,885,232       | 23,940,971   |


| Experiment:u128 | Set up input | Set up runtime | Interpreter loop | Total cycles |
|-----------------|--------------|----------------|------------------|--------------|
| Baseline        | --           | --             | --               | 1,204,947    |
| Wasmi           | 34,050       | 2,910          | 38,745,658       | 38,785,317   |
| Revm            | 43,473       | 10,961         | 1,002,027,202    | 1,002,088,904|
| Revm (optimized)| 33,817       | 10,961         | 237,555,380      | 237,603,026  | 

## Other notes

`fib-32.sol`,  `fib-64.sol`,  and `fib-128.sol` contain the smart contracts that are compiled to evm bytecode for revm. Wasmi doesn't support 128-bit integer types, so we substitute with u64. The Solidity optimizations for the revm benchmarks were suggested in [this](https://ethereum-magicians.org/t/long-term-l1-execution-layer-proposal-replace-the-evm-with-risc-v/23617/99) post. 

The moduli used are: 

| Experiment | Modulus                                           | 
|------------|---------------------------------------------------|
| u32        | 7919                                              |
| u64        | 65776547668456965                                 |
| u128       | 170141183460469231731687303715884105727           |

The wasmi version used is `0.42.1` and the SP1 version is `4.1.0`.    


