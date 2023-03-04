macro_rules! vec_box {
    ($($x:expr),*) => {
        vec![$(Box::new($x),)*]
    };
}

use std::{time::{Duration, Instant}, sync::Arc};

pub(crate) use vec_box;

pub fn benchmark_loop_for_singlethread(time_for_run: Duration, code: &Box<dyn Benchmark>) -> u128 {
    let start = Instant::now();
    let mut count = 0u128;

    loop {
        code.run_iter();

        count += 1;
        if start.elapsed() >= time_for_run {
            return count;
        }
    }
}

pub fn benchmark_loop_for_multithread<'a>(time_for_run: Duration, code: &'a Arc<&'a Box<dyn Benchmark + Send + Sync>>) -> u128 {
    enum Message {
        DoneOneJob
    }

    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let (tx, rx) = std::sync::mpsc::channel();

    let start = std::time::Instant::now();
    for _ in 0..9999999 {
        let tx = tx.clone();
        let code = code.clone();

        /* TODO 
        pool.execute(move || {
            code.run_iter();

            #[allow(unused_must_use)]
            {
                tx.send(Message::DoneOneJob);
            }
        });
        */
    }

    let mut points = 0u128;
    loop {
        match rx.recv().unwrap() {
            Message::DoneOneJob => points += 1
        }

        if start.elapsed() >= time_for_run {
            drop(pool);
            return points;
        }
    }
}

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

use super::Benchmark;
