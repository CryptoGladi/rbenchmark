# rbenchmark

![license](https://img.shields.io/github/license/CryptoGladi/rbenchmark?style=for-the-badge)
![code size](https://img.shields.io/github/languages/code-size/CryptoGladi/rbenchmark?style=for-the-badge)
![downloads](https://img.shields.io/crates/d/rbenchmark?style=for-the-badge)
![build](https://img.shields.io/github/actions/workflow/status/CryptoGladi/rbenchmark/rust.yml?branch=master&style=for-the-badge)

Library for evaluating the performance of your computer,  written in **pure Rust**

# Example :rocket:

```rust
use rbenchmark::prelude::*;

let mut runner = BenchmarkRunner::default();
let result = runner.run_all(|_progress | {}).unwrap();
println!("{:?}", result);
```

# Features :star:

* Write your own [`benchmarks`](crate::benchmark::Benchmark) and run them.
* Write your own runner for benchmarks
* Set the [`time`](crate::benchmark_runner::BenchmarkRunner::time_for_run_one_bench) of the benchmark
* Multi-core support
* Adjust which benchmarks to run
* Callback functions

# Example program

![render1678533482667](https://user-images.githubusercontent.com/116446344/224482340-5703a9b0-a246-4c4b-979b-b7c9c97b3c19.gif)
