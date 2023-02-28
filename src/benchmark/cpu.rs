use std::time::Instant;
use rand::Rng;
use rayon::prelude::*;

#[derive(Debug, Default)]
pub struct OneAndManyThread<T> {
    one: T,
    many: T
}

// TODO Arithmetic Compression Decompression Cryptography SIMD 
// https://github.com/L3tum/CPU-Benchmark

#[derive(Debug, Default)]
pub struct CPU {
    arithmetic: OneAndManyThread<u128>,
    compression: OneAndManyThread<u128>,
    decompression: OneAndManyThread<u128>,
    cryptography: OneAndManyThread<u128>,
    simd: u128,
    total_points: u128,
}

impl CPU {
    fn run_iter_math() -> OneAndManyThread<u128> {
        let some_value1 = rand::thread_rng().gen_range(342..10_000);
        let some_value2 = rand::thread_rng().gen_range(3..342);

        let now_one = Instant::now();
        (1..1_000_000_000).into_iter().for_each(|_| {
            let _test_sum = some_value1 + some_value2;
            let _test_sub = some_value1 - some_value2;
            let _test_div = some_value1 / some_value2;
            let _test_mul = some_value1 * some_value2;
        });
        let before_one = now_one.elapsed();


        let now_many = Instant::now();
        (1..1_000_000_000).into_par_iter().for_each(|_| {
            let _test_sum = some_value1 + some_value2;
            let _test_sub = some_value1 - some_value2;
            let _test_div = some_value1 / some_value2;
            let _test_mul = some_value1 * some_value2;
        });
        let before_many = now_many.elapsed();

        OneAndManyThread { one: before_one.as_millis(), many: before_many.as_millis() }
    }

    pub fn run_all_becnhmarks() -> Self {
        todo!()
    }
}

fn fcpu() {
    let cpu = CPU::run_all_becnhmarks();
    println!("{cpu:?}");
    println!("ss");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu() {
        let cpu = CPU::run_all_becnhmarks();
        println!("{cpu:?}");
        println!("ss");
    }
}