pub mod info;
pub mod runners;

use crate::benchmark::{macros::vec_box, Benchmark};
use info::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct BenchmarkRunner {
    /// [Benchmark](crate::benchmark::Benchmark) for run
    pub benchmarks: Vec<Box<dyn Benchmark>>,
    pub time_for_run_one_bench: Duration,

    /// Number of threads to be used
    pub num_cpus: usize,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        use crate::benchmark::cpu::prelude::*;
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

pub enum Progress<'a> {
    StartingSinglethreadBenchmark(&'a dyn Benchmark),
    DoneSinglethreadBenchmark(&'a dyn Benchmark),
    StartingMultithreadBenchmark(&'a dyn Benchmark),
    DoneMultithreadBenchmark(&'a dyn Benchmark),
}

pub type BenchmarkName<'a> = &'a str;

impl<'a> BenchmarkRunner {
    pub fn run_only_singlethread(
        &mut self,
        info: &mut HashMap<BenchmarkName, InfoOneBench>,
        mut callback: impl FnMut(Progress) + 'a,
    ) -> anyhow::Result<Duration> {
        let start = Instant::now();

        for bench in self.benchmarks.iter() {
            (callback)(Progress::StartingSinglethreadBenchmark(bench.as_ref()));

            let points = runners::benchmark_loop_for_singlethread(
                self.time_for_run_one_bench,
                bench.as_ref(),
            )?;

            info.entry(bench.name())
                .and_modify(|x| x.singlethread_points += points)
                .or_insert(InfoOneBench {
                    singlethread_points: points,
                    multithread_points: 0,
                });
            (callback)(Progress::DoneSinglethreadBenchmark(bench.as_ref()));
        }

        Ok(start.elapsed())
    }

    pub fn run_only_multithread(
        &mut self,
        info: &mut HashMap<BenchmarkName, InfoOneBench>,
        mut callback: impl FnMut(Progress) + 'a,
    ) -> anyhow::Result<Duration> {
        let start = Instant::now();

        for bench in self.benchmarks.iter() {
            (callback)(Progress::StartingMultithreadBenchmark(bench.as_ref()));
            let points = runners::benchmark_loop_for_multithread(
                self.time_for_run_one_bench,
                bench.as_ref(),
                self.num_cpus,
            )?;

            info.entry(bench.name())
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
        let mut info_for_any_bench: HashMap<BenchmarkName, InfoOneBench> =
            HashMap::with_capacity(self.benchmarks.len());

        let time_for_singlethread =
            self.run_only_singlethread(&mut info_for_any_bench, &mut callback)?;
        let time_for_multithread =
            self.run_only_multithread(&mut info_for_any_bench, &mut callback)?;

        Ok(Info {
            running_time: time_for_multithread + time_for_singlethread,
            info: info_for_any_bench,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_all() {
        let mut runner = BenchmarkRunner {
            time_for_run_one_bench: Duration::from_millis(1),
            ..Default::default()
        };

        runner.run_all(|_| {}).unwrap();
    }
}
