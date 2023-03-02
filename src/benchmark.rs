use std::time::Duration;

pub mod cpu;

pub trait Benchmark {
    // TODO CHANGE run() to next()?
    // Many copy paste code in macro benchmark_loop impl_benchmark
    fn run(&self, multithread_mode: bool, time_for_run: Duration) -> u128;
}

#[macro_export]
macro_rules! benchmark_loop {
    ($time_for_run:ident, $code:block) => {
        let start = std::time::Instant::now();
        let mut count = 0u128;

        loop {
            $code

            count += 1;
            if start.elapsed() >= $time_for_run {
                return count;
            }
        }
    };
}

pub use benchmark_loop;

#[macro_export]
macro_rules! impl_benchmark {
    ($struct:ident, $code:block) => {
        impl crate::benchmark::Benchmark for $struct {
            fn run(&self, multithread_mode: bool, time_for_run: std::time::Duration) -> u128 {
                crate::benchmark::benchmark_loop!(time_for_run, $code);
            }
        }
    };
}

pub use impl_benchmark;

pub struct BenchmarkRunner {
    benchmarks: Vec<Box<dyn Benchmark>>
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self {
            benchmarks: vec![Box::new(cpu::prelude::BenchmarkCompression::default())]
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_benchmark_runner() {
        let runner = BenchmarkRunner::default();
    }
}