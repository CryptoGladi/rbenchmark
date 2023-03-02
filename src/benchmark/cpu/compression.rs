use crate::benchmark::impl_benchmark;

#[derive(Debug, Default)]
pub struct BenchmarkCompression;

impl_benchmark!(BenchmarkCompression, {
    match multithread_mode {
        true => {

        },
        false => {

        }
    }
});