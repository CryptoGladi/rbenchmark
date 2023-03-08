use rbenchmark::prelude::*;

fn main() {
    let runner = BenchmarkRunner::default();

    println!("Start all benchmark!");
    let info = runner.run_all().unwrap();

    println!("Info about test: {:?}", info);
    println!("Total points: {}", info.total_points());
    println!(
        "Total multithread points: {}",
        info.total_multithread_points()
    );
    println!(
        "Total singlethread points: {}",
        info.total_singlethread_points()
    );
}
