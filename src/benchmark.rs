use std::time::{Duration, Instant};

pub mod cpu;

pub trait Benchmark {
    fn run_singlethread(&self, time_for_run: Duration) -> u128;

    fn run_multithread(&self, time_for_run: Duration) -> u128;

    fn name(&self) -> &'static str;
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
    ($struct:ident, singlethread: $code:block, multithread: $code_multithread:block) => {
        impl crate::benchmark::Benchmark for $struct {
            fn run_singlethread(&self, time_for_run: std::time::Duration) -> u128 {
                crate::benchmark::benchmark_loop!(time_for_run, $code);
            }

            fn run_multithread(&self, time_for_run: std::time::Duration) -> u128 {
                crate::benchmark::benchmark_loop!(time_for_run, $code_multithread);
            }
        }
    };
}

pub use impl_benchmark;

pub struct BenchmarkRunner {
    benchmarks: Vec<Box<dyn Benchmark>>,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self {
            benchmarks: vec![Box::new(cpu::prelude::BenchmarkCompression::default())],
        }
    }
}

#[derive(Debug)]
pub struct Points {
    pub singlethread: u128,
    pub multithread: u128,
    pub time: Duration,
}

pub enum Progress<'a> {
    StartingSinglethreadBenchmark(&'a Box<dyn Benchmark>),
    DoneSinglethreadBenchmark(&'a Box<dyn Benchmark>),
    StartingMultithreadBenchmark(&'a Box<dyn Benchmark>),
    DoneMultithreadBenchmark(&'a Box<dyn Benchmark>),
}

impl BenchmarkRunner {
    pub fn run_all(&self, time_for_run_one_bench: Duration) -> Points {
        let start = Instant::now();

        let mut points_singlethread = 0;
        self.benchmarks
            .iter()
            .for_each(|bench| points_singlethread += bench.run_singlethread(time_for_run_one_bench));

        let mut points_multithread = 0;
        self.benchmarks
            .iter()
            .for_each(|bench| points_multithread += bench.run_multithread(time_for_run_one_bench));

        return Points {
            multithread: points_multithread,
            singlethread: points_singlethread,
            time: start.elapsed(),
        };
    }

    pub fn run_all_with_callback(
        &self,
        time_for_run_one_bench: Duration,
        mut callback: impl FnMut(Progress),
    ) -> Points {
        let start = Instant::now();

        let mut points_singlethread = 0;
        self.benchmarks.iter().for_each(|bench| {
            callback(Progress::StartingSinglethreadBenchmark(bench));
            points_singlethread += bench.run_singlethread(time_for_run_one_bench);
            callback(Progress::DoneSinglethreadBenchmark(bench));
        });

        let mut points_multithread = 0;
        self.benchmarks.iter().for_each(|bench| {
            callback(Progress::StartingMultithreadBenchmark(bench));
            points_multithread += bench.run_multithread(time_for_run_one_bench);
            callback(Progress::DoneMultithreadBenchmark(bench));
        });

        return Points {
            multithread: points_multithread,
            singlethread: points_singlethread,
            time: start.elapsed(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_benchmark_runner() {
        let runner = BenchmarkRunner::default();
        runner.run_all(Duration::from_secs(1));
    }
}
