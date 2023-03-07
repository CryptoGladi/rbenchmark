use crate::benchmark::Benchmark;

#[derive(Debug)]
pub struct BenchmarkArithmetic {}

impl Default for BenchmarkArithmetic {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl Benchmark for BenchmarkArithmetic {
    fn run_iter(&self) {
        let double_for_work: (f64, f64) = (rand::random(), rand::random());
        let    float_for_work: (f32, f32) = (rand::random(), rand::random());
        let    int_for_work: (i32, i32) = (rand::random(), rand::random());
        let    big_int_for_work:(isize, isize) = (rand::random(), rand::random());

        let _ = double_for_work.0 + double_for_work.1;
        let _ = float_for_work.0 + float_for_work.1;
        let _ = int_for_work.0 + int_for_work.1;
        let _ = big_int_for_work.0 + big_int_for_work.1;
    }

    fn name(&self) -> &'static str {
        "arithmetic"
    }
}