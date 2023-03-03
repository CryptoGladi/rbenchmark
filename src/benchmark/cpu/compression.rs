use crate::{benchmark::Benchmark, benchmark_loop};
use rand::prelude::*;
use std::{
    sync::{mpsc::channel, Arc},
    time::Instant,
};
use threadpool::ThreadPool;

#[derive(Debug)]
pub struct BenchmarkCompression {
    pub size_byte_for_compession: u128,
    buffer_for_compession: Vec<u8>,
}

impl Default for BenchmarkCompression {
    fn default() -> Self {
        let size_byte_for_compession = byte_unit::n_mb_bytes!(1);

        let mut rng = rand::thread_rng();
        let mut buffer_for_compession = vec![0u8; size_byte_for_compession as usize];
        buffer_for_compession.shuffle(&mut rng);

        Self {
            size_byte_for_compession,
            buffer_for_compession,
        }
    }
}

enum Message {
    DoneOneJob,
}

impl Benchmark for BenchmarkCompression {
    fn run_singlethread(&self, time_for_run: std::time::Duration) -> u128 {
        benchmark_loop!(time_for_run, {
            let _ = lzma::compress(&self.buffer_for_compession[..], lzma::EXTREME_PRESET).unwrap();
        });
    }

    fn run_multithread(&self, time_for_run: std::time::Duration) -> u128 {
        let pool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = channel();

        let start = Instant::now();
        let buffer_for_compession = Arc::new(self.buffer_for_compession.clone());
        for _ in 0..9999999 {
            let tx = tx.clone();
            let buffer_for_compession = buffer_for_compession.clone();

            pool.execute(move || {
                let _ = lzma::compress(&buffer_for_compession[..], lzma::EXTREME_PRESET).unwrap();
                tx.send(Message::DoneOneJob).unwrap();
            });
        }

        let mut points = 0u128;
        loop {
            match rx.recv().unwrap() {
                Message::DoneOneJob => points += 1,
            }

            if start.elapsed() >= time_for_run {
                return points;
            }
        }
    }

    fn name(&self) -> &'static str {
        "compression"
    }
}
