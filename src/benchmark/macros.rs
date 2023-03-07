use super::Benchmark;
use std::time::{Duration, Instant};

macro_rules! vec_box {
    ($($x:expr),*) => {
        vec![$(Box::new($x),)*]
    };
}

pub(crate) use vec_box;

pub fn benchmark_loop_for_singlethread(time_for_run: Duration, bench: &dyn Benchmark) -> u128 {
    let start = Instant::now();
    let mut count = 0u128;

    loop {
        bench.run_iter();

        count += 1;
        if start.elapsed() >= time_for_run {
            return count;
        }
    }
}

pub fn benchmark_loop_for_multithread(
    time_for_run: Duration,
    bench: &dyn Benchmark,
    num_cpus: usize,
) -> u128 {
    let mut count = 0;
    let (done_job, run_job) = crossbeam::channel::unbounded();
    let start = Instant::now();

    let code_for_thread = || {
        bench.run_iter();
        done_job.send(1).unwrap();
    };

    crossbeam::scope(|s| {
        for _ in 0..num_cpus {
            s.spawn(|_| {
                code_for_thread();
            });
        }

        loop {
            if run_job.recv().unwrap() == 1 {
                count += 1;

                if start.elapsed() >= time_for_run {
                    break;
                }

                s.spawn(|_| {
                    code_for_thread();
                });
            }
        }
    })
    .unwrap();

    count
}
