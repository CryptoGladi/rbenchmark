use std::sync::Arc;
use crate::{
    benchmark::Benchmark, benchmark_loop_for_multithread, benchmark_loop_for_singlethread,
};
use rand::prelude::*;

#[derive(Debug)]
pub struct BenchmarkCompression {
    pub size_byte_for_compession: u128,
    pub preset: u32,
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
            preset: lzma::EXTREME_PRESET,
            buffer_for_compession,
        }
    }
}

impl Benchmark for BenchmarkCompression {
    fn run_singlethread(&self, time_for_run: std::time::Duration) -> u128 {
        benchmark_loop_for_singlethread!(time_for_run, {
            let i = lzma::compress(&self.buffer_for_compession[..], self.preset).unwrap();
            drop(i);
        });
    }

    fn run_multithread(&self, time_for_run: std::time::Duration) -> u128 {
        let buffer_for_compession = Arc::new(self.buffer_for_compession.clone());
        let preset = Arc::new(self.preset.clone());

        benchmark_loop_for_multithread!(
            time_for_run,
            code: {
                let i = lzma::compress(&buffer_for_compession[..], *preset).unwrap();
                drop(i);
            },
            move: buffer_for_compession, preset
        );
    }

    fn name(&self) -> &'static str {
        "compression"
    }
}
