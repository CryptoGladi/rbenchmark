use indicatif::{ProgressBar, ProgressStyle};
use rbenchmark::benchmark::Benchmark;
use rbenchmark::prelude::*;
use std::time::Duration;

fn main() {
    let mut runner = BenchmarkRunner::default();
    let count_bench = runner.benchmarks.len() * 2;

    let pb = ProgressBar::new(count_bench as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(200));
    pb.reset();

    let change_name_bench = |bench: &dyn Benchmark, is_singlethread: bool| {
        pb.inc(1);

        pb.set_message(match is_singlethread {
            true => format!("singlethread bench: {}", bench.name()),
            false => format!("multithread bench: {}", bench.name())
        });
    };

    let info = runner
        .run_all(|progress| match progress {
            DoneSinglethreadBenchmark(bench) => change_name_bench(bench, true),
            DoneMultithreadBenchmark(bench) => change_name_bench(bench, false),
            _ => {}
        })
        .unwrap();
    pb.finish();

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
