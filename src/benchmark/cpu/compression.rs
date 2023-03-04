use crate::{
    benchmark::Benchmark, 
};
use rand::prelude::*;
use std::{
    io::{BufReader, BufWriter, Cursor},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct StorageBuffer {
    buffer: Vec<u8>,
}

impl StorageBuffer {
    fn new(size_byte_for_compession: u128) -> Self {
        let mut rng = rand::thread_rng();
        let mut buffer = vec![0u8; size_byte_for_compession as usize];
        buffer.shuffle(&mut rng);

        Self {
            buffer
        }
    }
}

impl StorageBuffer {
    fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}

#[derive(Debug)]
pub struct BenchmarkCompression {
    pub size_byte_for_compession: u128,
    buffer_for_compession: StorageBuffer,
}

impl Default for BenchmarkCompression {
    fn default() -> Self {
        let size = byte_unit::n_mb_bytes!(1);

        Self {
            size_byte_for_compession: size,
            buffer_for_compession: StorageBuffer::new(size)
        }
    }
}

impl Benchmark for BenchmarkCompression {
    fn run_iter(&self) {
        let mut reader_buf = BufReader::new(Cursor::new(self.buffer_for_compession.get_buffer()));
        let mut writer_buf = BufWriter::new(vec![]);

        lzma_rs::lzma_compress(&mut reader_buf, &mut writer_buf).unwrap();
    }

    fn name(&self) -> &'static str {
        "compression"
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

}