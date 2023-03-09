use self::macros::vec_box;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub mod cpu;
pub mod macros;

pub trait Benchmark: Send + Sync {
    fn run_iter(&self) -> anyhow::Result<()>;

    fn name(&self) -> &'static str;
}

pub struct BenchmarkRunner {
    pub benchmarks: Vec<Box<dyn Benchmark>>,
    pub time_for_run_one_bench: Duration,
    pub num_cpus: usize,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        use cpu::prelude::*;
        Self {
            benchmarks: vec_box![
                BenchmarkCompression::default(),
                BenchmarkDecompression::default(),
                BenchmarkCryptography::default(),
                BenchmarkDatabase::default()
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

impl<'a> BenchmarkRunner {
    pub fn run_only_singlethread(&mut self, info: &mut HashMap<&str, InfoOneBench>, mut callback: impl FnMut(Progress) + 'a) -> anyhow::Result<Duration> {
        let start = Instant::now();

        for bench in self.benchmarks.iter() {
            (callback)(Progress::StartingSinglethreadBenchmark(bench.as_ref()));

            let points = macros::benchmark_loop_for_singlethread(
                self.time_for_run_one_bench,
                bench.as_ref(),
            )?;

            info
                .entry(bench.name())
                .and_modify(|x| x.singlethread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: points,
                    multithread_points: 0,
                });
            (callback)(Progress::DoneSinglethreadBenchmark(bench.as_ref()));
        }

        Ok(start.elapsed())
    }

    pub fn run_only_multithread(&mut self, info: &mut HashMap<&str, InfoOneBench>, mut callback: impl FnMut(Progress) + 'a) -> anyhow::Result<Duration> {
        let start = Instant::now();

    for bench in self.benchmarks.iter() {
        (callback)(Progress::StartingMultithreadBenchmark(bench.as_ref()));
        let points = macros::benchmark_loop_for_multithread(
            self.time_for_run_one_bench,
            bench.as_ref(),
            self.num_cpus,
        )?;

        info
            .entry(bench.name())
            .and_modify(|x| x.multithread_points += points)
            .or_insert(InfoOneBench {
                singlethread_points: 0,
                multithread_points: points,
            });
        (callback)(Progress::DoneMultithreadBenchmark(bench.as_ref()));
    }

    Ok(start.elapsed())
    }

    pub fn run_all(&mut self, mut callback: impl FnMut(Progress) + 'a) -> anyhow::Result<Info> {
        let mut info_for_any_bench: HashMap<&str, InfoOneBench> =
            HashMap::with_capacity(self.benchmarks.len());
            
        let time_for_singlethread = self.run_only_singlethread(&mut info_for_any_bench, &mut callback)?;
        let time_for_multithread = self.run_only_multithread(&mut info_for_any_bench, &mut callback)?;

        Ok(Info {
            time: time_for_multithread + time_for_singlethread,
            info: info_for_any_bench,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_benchmark_runner() {
        let mut runner = BenchmarkRunner {
            time_for_run_one_bench: Duration::from_millis(1),
            ..Default::default()
        };

        runner.run_all(|_| {}).unwrap();

        //let i = runner.run_all().unwrap();
    }
}
