use std::sync::Arc;
use crate::{
    benchmark::Benchmark, benchmark_loop_for_multithread, benchmark_loop_for_singlethread,
};
use rand::prelude::*;

#[derive(Debug)]
pub struct BenchmarkDecompression {
    pub size_byte_for_decompession: u128,
    pub preset: u32,
    buffer_for_decompession: Vec<u8>,
}

impl Default for BenchmarkDecompression {
    fn default() -> Self {
        let preset = lzma::EXTREME_PRESET;

        let size_byte_for_decompession = byte_unit::n_mb_bytes!(1);
        let mut rng = rand::thread_rng();
        let mut buffer_for_compession = vec![0u8; size_byte_for_decompession as usize];
        buffer_for_compession.shuffle(&mut rng);

        let buffer_for_decompession = lzma::compress(&buffer_for_compession[..], preset).unwrap();

        Self {
            size_byte_for_decompession,
            preset,
            buffer_for_decompession 
        }
    }
}

impl Benchmark for BenchmarkDecompression {
    fn run_singlethread(&self, time_for_run: std::time::Duration) -> u128 {
        benchmark_loop_for_singlethread!(time_for_run, {
            let i = lzma::decompress(&self.buffer_for_decompession[..]).unwrap();
            drop(i);
        });
    }

    fn run_multithread(&self, time_for_run: std::time::Duration) -> u128 {
        let buffer_for_decompession = Arc::new(self.buffer_for_decompession.clone());

        benchmark_loop_for_multithread!(
            time_for_run,
            code: {
                let i = lzma::decompress(&buffer_for_decompession[..]).unwrap();
                drop(i);
            },
            move: buffer_for_decompession
        );
    }

    fn name(&self) -> &'static str {
        "decompression"
    }
}