use std::{sync::Arc, io::Cursor};
use crate::{
    benchmark::Benchmark, benchmark_loop_for_multithread, benchmark_loop_for_singlethread,
};
use rand::prelude::*;

#[derive(Debug)]
pub struct BenchmarkDecompression {
    pub size_byte_for_decompession: u128,
    buffer_for_decompession: Vec<u8>,
}

impl Default for BenchmarkDecompression {
    fn default() -> Self {
        let size_byte_for_decompession = byte_unit::n_mb_bytes!(1);
        let mut rng = rand::thread_rng();
        let mut buffer_for_compession = vec![0u8; size_byte_for_decompession as usize];
        buffer_for_compession.shuffle(&mut rng);

        let mut buffer_for_decompession = vec![];
        lzma_rs::lzma_compress(&mut Cursor::new(buffer_for_compession), &mut buffer_for_decompession).unwrap();

        Self {
            size_byte_for_decompession,
            buffer_for_decompession 
        }
    }
}

impl Benchmark for BenchmarkDecompression {
    fn run_singlethread(&self, time_for_run: std::time::Duration) -> u128 {
        benchmark_loop_for_singlethread!(time_for_run, {
            let mut output = vec![];
            lzma_rs::lzma_decompress(&mut Cursor::new(&self.buffer_for_decompession), &mut output).unwrap();
        });
    }

    fn run_multithread(&self, time_for_run: std::time::Duration) -> u128 {
        let buffer_for_decompession = Arc::new(self.buffer_for_decompession.clone());

        benchmark_loop_for_multithread!(
            time_for_run,
            code: {
                let mut output = vec![];
                lzma_rs::lzma_decompress(&mut Cursor::new(&*buffer_for_decompession), &mut output).unwrap();
            },
            move: buffer_for_decompession
        );
    }

    fn name(&self) -> &'static str {
        "decompression"
    }
}