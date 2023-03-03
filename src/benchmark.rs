use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use self::macros::vec_box;

pub mod cpu;
pub mod macros;

pub trait Benchmark {
    fn run_singlethread(&self, time_for_run: Duration) -> u128;

    fn run_multithread(&self, time_for_run: Duration) -> u128;

    fn name(&self) -> &'static str;
}

pub struct BenchmarkRunner {
    benchmarks: Vec<Box<dyn Benchmark>>,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        use cpu::prelude::*;

        Self {
            benchmarks: vec_box![BenchmarkCompression::default(), BenchmarkDecompression::default()],
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
    StartingSinglethreadBenchmark(&'a Box<dyn Benchmark>),
    DoneSinglethreadBenchmark(&'a Box<dyn Benchmark>),
    StartingMultithreadBenchmark(&'a Box<dyn Benchmark>),
    DoneMultithreadBenchmark(&'a Box<dyn Benchmark>),
}

impl BenchmarkRunner {
    pub fn run_all(&self, time_for_run_one_bench: Duration) -> Info {
        let start = Instant::now();
        let mut info_for_any_bench: HashMap<&str, InfoOneBench> =
            HashMap::with_capacity(self.benchmarks.len() * 2);

        self.benchmarks.iter().for_each(|bench| {
            let points = bench.run_singlethread(time_for_run_one_bench);
            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.singlethread_points += points)
                .or_default();
            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.singlethread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: points,
                    multithread_points: 0,
                });
        });

        self.benchmarks.iter().for_each(|bench| {
            let points = bench.run_multithread(time_for_run_one_bench);
            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.multithread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: 0,
                    multithread_points: points,
                });
        });

        return Info {
            time: start.elapsed(),
            info: info_for_any_bench,
        };
    }

    pub fn run_all_with_callback(
        &self,
        time_for_run_one_bench: Duration,
        mut callback: impl FnMut(Progress),
    ) -> Info {
        let start = Instant::now();
        let mut info_for_any_bench: HashMap<&str, InfoOneBench> =
            HashMap::with_capacity(self.benchmarks.len() * 2);

        self.benchmarks.iter().for_each(|bench| {
            callback(Progress::StartingSinglethreadBenchmark(bench));
            let points = bench.run_singlethread(time_for_run_one_bench);
            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.singlethread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: points,
                    multithread_points: 0,
                });
            callback(Progress::DoneSinglethreadBenchmark(bench));
        });

        self.benchmarks.iter().for_each(|bench| {
            callback(Progress::StartingMultithreadBenchmark(bench));
            let points = bench.run_multithread(time_for_run_one_bench);
            info_for_any_bench
                .entry(bench.name())
                .and_modify(|x| x.multithread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: 0,
                    multithread_points: points,
                });
            callback(Progress::DoneMultithreadBenchmark(bench));
        });

        return Info {
            time: start.elapsed(),
            info: info_for_any_bench,
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
