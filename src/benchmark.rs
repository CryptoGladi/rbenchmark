use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use self::macros::vec_box;

pub mod cpu;
pub mod macros;

pub trait Benchmark: Send + Sync {
    fn run_iter(&self) -> anyhow::Result<()>;

    fn name(&self) -> &'static str;
}

pub struct BenchmarkRunner {
    benchmarks: Vec<Box<dyn Benchmark>>,
    time_for_run_one_bench: Duration,
    num_cpus: usize,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        use cpu::prelude::*;

        Self {
            benchmarks: vec_box![
                BenchmarkCompression::default(),
                BenchmarkDecompression::default(),
                BenchmarkCryptography::default()
            ],
            time_for_run_one_bench: Duration::from_secs(5),
            num_cpus: num_cpus::get(),
        }
    }
}

#[derive(Debug, Default)]
pub struct InfoOneBench {
    singlethread_points: u128,
    multithread_points: u128,
}

#[derive(Debug)]
pub struct Info<'a> {
    pub time: Duration,
    pub info: HashMap<&'a str, InfoOneBench>,
}

impl Info<'_> {
    pub fn total_singlethread_points(&self) -> u128 {
        self.info.iter().map(|x| x.1.singlethread_points).sum()
    }

    pub fn total_multithread_points(&self) -> u128 {
        self.info.iter().map(|x| x.1.multithread_points).sum()
    }

    pub fn total_points(&self) -> u128 {
        self.total_multithread_points() + self.total_singlethread_points()
    }
}

pub enum Progress<'a> {
    StartingSinglethreadBenchmark(&'a dyn Benchmark),
    DoneSinglethreadBenchmark(&'a dyn Benchmark),
    StartingMultithreadBenchmark(&'a dyn Benchmark),
    DoneMultithreadBenchmark(&'a dyn Benchmark),
}

impl BenchmarkRunner {
    pub fn run_all(&self) -> anyhow::Result<Info> {
        let start = Instant::now();
        let mut info_for_any_bench: HashMap<&str, InfoOneBench> =
            HashMap::with_capacity(self.benchmarks.len() * 2);

        for bench in self.benchmarks.iter() {
            let points = macros::benchmark_loop_for_singlethread(
                self.time_for_run_one_bench,
                bench.as_ref(),
            )?;

            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.singlethread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: points,
                    multithread_points: 0,
                });
        }

        for bench in self.benchmarks.iter() {
            let points = macros::benchmark_loop_for_multithread(
                self.time_for_run_one_bench,
                bench.as_ref(),
                self.num_cpus,
            )?;

            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.multithread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: 0,
                    multithread_points: points,
                });
        }

        Ok(Info {
            time: start.elapsed(),
            info: info_for_any_bench,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_benchmark_runner() {
        let runner = BenchmarkRunner {
            time_for_run_one_bench: Duration::from_millis(1),
            ..Default::default()
        };

        runner.run_all();
    }
}
