use crate::benchmark::Benchmark;
use std::sync::Mutex;

pub struct BenchmarkDatabase {
    pub sqlite: Mutex<sqlite::Connection>,
}

impl Default for BenchmarkDatabase {
    fn default() -> Self {
        let sqlite = sqlite::open(":memory:").unwrap();

        sqlite
            .execute(
                "CREATE TABLE Persons (
            PersonID int,
            LastName varchar(255),
            FirstName varchar(255),
            Address varchar(255),
            City varchar(255)
        );",
            )
            .unwrap();

        Self {
            sqlite: Mutex::new(sqlite),
        }
    }
}

impl Benchmark for BenchmarkDatabase {
    fn run_iter(&self) -> anyhow::Result<()> {
        for _ in 0..4000 {
            self.sqlite.lock().unwrap().execute("INSERT INTO Persons (PersonID, LastName, FirstName, Address, City) VALUES ('421', 'Crypto', 'Gladi', 'USA', 'Penza')")?;
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "database"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_iter() {
        let bench = BenchmarkDatabase::default();
        bench.run_iter().unwrap();
    }
}
