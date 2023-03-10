//! rbenchmark is a library for evaluating the performance of your computer, written in **pure Rust**
//!
//! # Example
//!
//! ```no_run
//! use rbenchmark::prelude::*;
//! # use std::time::Duration;
//!
//! let mut runner = BenchmarkRunner::default(); // is slow!
//! # runner.time_for_run_one_bench = Duration::from_millis(1);
//!
//! let result = runner.run_all(|_progress | {}).unwrap();
//! println!("{:?}", result);
//! ```
//!
//! # Features
//!
//! * Write your own [`benchmarks`](crate::benchmark::Benchmark) and run them.
//! * Write your own runner for benchmarks
//! * Set the [`time`](crate::benchmark_runner::BenchmarkRunner::time_for_run_one_bench) of the benchmark
//! * Multi-core support
//! * Adjust which benchmarks to run
//! * Callback functions

pub mod benchmark;
pub mod benchmark_runner;
pub mod prelude;
