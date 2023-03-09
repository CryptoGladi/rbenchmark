use crate::benchmark::Benchmark;
use std::io::{BufReader, BufWriter, Cursor};

#[derive(Debug)]
pub struct BenchmarkCompression {
    pub buffer_for_compession: Vec<u8>,
}

impl Default for BenchmarkCompression {
    fn default() -> Self {
        Self {
            buffer_for_compession: vec![0u8; byte_unit::n_mb_bytes!(1) as usize],
        }
    }
}

impl Benchmark for BenchmarkCompression {
    fn run_iter(&self) -> anyhow::Result<()> {
        let mut reader_buf = BufReader::new(Cursor::new(&self.buffer_for_compession));
        let mut writer_buf = BufWriter::new(vec![]);

        lzma_rs::lzma_compress(&mut reader_buf, &mut writer_buf)?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "compression"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_iter() {
        let bench = BenchmarkCompression::default();
        bench.run_iter().unwrap();
    }
}
