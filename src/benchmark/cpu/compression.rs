use std::{sync::Arc, io::{BufReader, Cursor, BufWriter}};
use crate::{
    benchmark::Benchmark, benchmark_loop_for_multithread, benchmark_loop_for_singlethread,
};
use rand::prelude::*;

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

impl Benchmark for BenchmarkCompression {
    fn run_singlethread(&self, time_for_run: std::time::Duration) -> u128 {
        let mut reader_buf = BufReader::new(Cursor::new(self.buffer_for_compession.clone()));
        let mut writer_buf = BufWriter::new(vec![]);
        
        benchmark_loop_for_singlethread!(time_for_run, {
            lzma_rs::lzma_compress(&mut reader_buf, &mut writer_buf).unwrap();
        });
    }

    fn run_multithread(&self, time_for_run: std::time::Duration) -> u128 {
        let mut reader_buf = Arc::new(BufReader::new(Cursor::new(self.buffer_for_compession.clone())));

        benchmark_loop_for_multithread!(
            time_for_run,
            code: {
                let mut writer_buf = BufWriter::new(vec![]);
                unsafe {
                let mut ii = Arc::get_mut(&mut reader_buf);
                lzma_rs::lzma_compress(&mut ii, &mut writer_buf);
                }
            },
            move: reader_buf
        );
    }

    fn name(&self) -> &'static str {
        "compression"
    }
}
