//! Trait for own benchmarks

pub mod cpu;
pub mod macros;

/// Trait for own benchmarks
///
/// Use **[core::marker::Send]** and **[core::marker::Sync]**
///
/// # Example
///
/// ```no_run
/// use rbenchmark::prelude::*;
///
/// #[derive(Default)]
/// pub struct MyBenchmark {
///     // Some data
/// }
///
/// impl Benchmark for MyBenchmark {
///     fn run_iter(&self) -> anyhow::Result<()> {
///         // Work...
///
///         Ok(())
///     }
///
///     fn name(&self) -> &'static str {
///         "my benchmark"
///     }
/// }
/// ```
pub trait Benchmark: Send + Sync {
    /// After each cycle of this code, you will add one point
    fn run_iter(&self) -> anyhow::Result<()>;

    /// Return name benchmark
    fn name(&self) -> &'static str;
}
