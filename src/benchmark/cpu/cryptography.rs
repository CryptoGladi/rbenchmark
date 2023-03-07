use crate::benchmark::Benchmark;
use rand::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
pub struct BenchmarkCryptography;

impl Benchmark for BenchmarkCryptography {
    fn run_iter(&self) {
        // failed to set up alternative stack guard page
        // Resource temporarily unavailable
        // https://github.com/rust-lang/rust/issues/78497

        let mut rng = rand::thread_rng();
        let mut buffer = vec![0u8; byte_unit::n_mb_bytes!(1) as usize];
        buffer.shuffle(&mut rng);

        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let _ = hasher.finalize();
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
        bench.run_iter();
    }
}
