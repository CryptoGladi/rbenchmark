use super::Benchmark;
use std::time::{Duration, Instant};

macro_rules! vec_box {
    ($($x:expr),*) => {
        vec![$(Box::new($x),)*]
    };
}

use crossbeam::channel::TryRecvError;
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
    let (stop_job, job_is_stopped) = crossbeam::channel::unbounded::<()>();
    let start = Instant::now();

    crossbeam::scope(|s| { // TODO Лучше создать все потоки, а потом тупо ждать приказа завершения
        for _ in 0..num_cpus {
            s.spawn(|_| {
                loop {
                    bench.run_iter();
                    done_job.send(1).unwrap();

                    if let Err(TryRecvError::Disconnected) = job_is_stopped.try_recv() {
                        break;
                    }
                }
            });
        }

        loop {
            if run_job.recv().unwrap() == 1 {
                count += 1;

                if start.elapsed() >= time_for_run {
                    drop(stop_job);
                    break;
                }
            }
        }
    })
    .unwrap();

    count
}
