pub mod arithmetic;
pub mod compression;
pub mod cryptography;
pub mod decompression;
pub mod prelude;
pub mod simd;

#[derive(Debug, Default)]
pub struct OneAndManyThread<T> {
    one: T,
    many: T,
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
