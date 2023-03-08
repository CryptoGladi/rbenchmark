use crate::benchmark::Benchmark;

pub struct BenchmarkDatabase;

impl Default for BenchmarkDatabase {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl Benchmark for BenchmarkDatabase {
    fn run_iter(&self) -> anyhow::Result<()> {
        let sqlite = sqlite::open(":memory:")?;
        sqlite.execute("")?; // TODO

        Ok(())
    }

    fn name(&self) -> &'static str {
        "database"
    }
}