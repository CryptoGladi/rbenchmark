```
           /$$                                     /$$                                         /$$      
          | $$                                    | $$                                        | $$      
  /$$$$$$ | $$$$$$$   /$$$$$$  /$$$$$$$   /$$$$$$$| $$$$$$$  /$$$$$$/$$$$   /$$$$$$   /$$$$$$ | $$   /$$
 /$$__  $$| $$__  $$ /$$__  $$| $$__  $$ /$$_____/| $$__  $$| $$_  $$_  $$ |____  $$ /$$__  $$| $$  /$$/
| $$  \__/| $$  \ $$| $$$$$$$$| $$  \ $$| $$      | $$  \ $$| $$ \ $$ \ $$  /$$$$$$$| $$  \__/| $$$$$$/ 
| $$      | $$  | $$| $$_____/| $$  | $$| $$      | $$  | $$| $$ | $$ | $$ /$$__  $$| $$      | $$_  $$ 
| $$      | $$$$$$$/|  $$$$$$$| $$  | $$|  $$$$$$$| $$  | $$| $$ | $$ | $$|  $$$$$$$| $$      | $$ \  $$
|__/      |_______/  \_______/|__/  |__/ \_______/|__/  |__/|__/ |__/ |__/ \_______/|__/      |__/  \__/
```                           

> Library for evaluating the performance of your computer,  written in **pure Rust**

# Example

```rust
use rbenchmark::prelude::*;

let mut runner = BenchmarkRunner::default();
let result = runner.run_all(|_progress | {}).unwrap();
println!("{:?}", result);
```

# Features

* Write your own [`benchmarks`](crate::benchmark::Benchmark) and run them.
* Write your own runner for benchmarks
* Set the [`time`](crate::benchmark_runner::BenchmarkRunner::time_for_run_one_bench) of the benchmark
* Multi-core support
* Adjust which benchmarks to run
* Callback functions
