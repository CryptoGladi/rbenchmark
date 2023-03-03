use std::time::Duration;

use rbenchmark::prelude::*;

fn main() {
    let runner = BenchmarkRunner::default();

    println!("Start all benchmark!");
    let points = runner.run_all_with_callback(Duration::from_secs(5), |progress| match progress {
        StartingSinglethreadBenchmark(bench) => println!("Starting singlethread benchmark: {}...", bench.name()),
        DoneSinglethreadBenchmark(bench) => println!("Done singlethread benchmark: {}", bench.name()),
        StartingMultithreadBenchmark(bench) => println!("Starting multithread benchmark: {}...", bench.name()),
        DoneMultithreadBenchmark(bench) => println!("Done multithread benchmark: {}", bench.name())
    });

    println!("Info about test: {:?}", points)
}
