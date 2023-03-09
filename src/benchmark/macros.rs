use super::Benchmark;
use std::{
    time::{Duration, Instant},
};

macro_rules! vec_box {
    ($($x:expr),*) => {
        vec![$(Box::new($x),)*]
    };
}

pub(crate) use vec_box;

pub fn benchmark_loop_for_singlethread(
    time_for_run: Duration,
    bench: &dyn Benchmark,
) -> anyhow::Result<u128> {
    let start = Instant::now();
    let mut count = 0u128;

    loop {
        bench.run_iter()?;

        count += 1;
        if start.elapsed() >= time_for_run {
            return Ok(count);
        }
    }
}

pub fn benchmark_loop_for_multithread(
    time_for_run: Duration,
    bench: &dyn Benchmark,
    num_cpus: usize,
) -> anyhow::Result<u128> {
    let mut count = 0;
    let (done_job, run_job) = crossbeam::channel::unbounded();
    let start = Instant::now();

    let result = crossbeam::scope(|s| {
        for _ in 0..num_cpus {
            s.spawn(|_| loop {
                bench.run_iter().unwrap();
                if done_job.send(1).is_err() {
                    break;
                }
            });
        }

        loop {
            if run_job.recv().unwrap() == 1 {
                count += 1;

                if start.elapsed() >= time_for_run {
                    drop(run_job);
                    break;
                }
            }
        }
    });

    if let Err(e) = result {
        anyhow::bail!("error benchmark: {:?}", e);
    }

    Ok(count)
}
