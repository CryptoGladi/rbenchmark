use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rbenchmark::prelude::*;
use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

fn main() {
    let mut runner = BenchmarkRunner::default();
    let count_bench = runner.benchmarks.len() * 2;

    let mut pb = ProgressBar::new(count_bench as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-"));

    runner.callback = Box::new(|progress| {
        match progress {
            DoneSinglethreadBenchmark(bench) => { pb.inc(1); },
            DoneMultithreadBenchmark(bench) => todo!(),
            _ => {},
        }
    });

    pb.set_message(format!("bench: {}", 2 + 1));
    pb.finish();

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
