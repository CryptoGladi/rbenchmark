use crate::benchmark::Benchmark;
use rand::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct BenchmarkCryptography {
    pub buffer_for_hashing: Vec<u8>,
}

impl Default for BenchmarkCryptography {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut buffer = vec![0u8; byte_unit::n_mb_bytes!(25) as usize];
        buffer.shuffle(&mut rng);

        Self {
            buffer_for_hashing: buffer,
        }
    }
}

impl Benchmark for BenchmarkCryptography {
    fn run_iter(&self) -> anyhow::Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(&self.buffer_for_hashing);
        let _ = hasher.finalize();
        Ok(())
    }

    fn name(&self) -> &'static str {
        "cryptography"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_iter() {
        let bench = BenchmarkCryptography::default();
        bench.run_iter().unwrap();
    }
}
