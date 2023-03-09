use crate::benchmark::Benchmark;

#[derive(Debug, Default)]
pub struct BenchmarkDatabase;

impl Benchmark for BenchmarkDatabase {
    fn run_iter(&self) -> anyhow::Result<()> {
        let sqlite = sqlite::open(":memory:")?;
        sqlite.execute(
            "CREATE TABLE Persons (
            PersonID int,
            LastName varchar(255),
            FirstName varchar(255),
            Address varchar(255),
            City varchar(255)
        );",
        )?;
        sqlite.execute("INSERT INTO Persons (PersonID, LastName, FirstName, Address, City) VALUES ('421', 'Crypto', 'Gladi', 'USA', 'Penza')")?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "database"
    }
}
