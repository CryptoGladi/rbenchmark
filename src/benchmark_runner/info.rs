use super::BenchmarkName;
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Default)]
pub struct InfoOneBench {
    pub singlethread_points: u128,
    pub multithread_points: u128,
}

#[derive(Debug)]
pub struct Info<'a> {
    pub running_time: Duration,
    pub info: HashMap<BenchmarkName<'a>, InfoOneBench>,
}

impl Info<'_> {
    pub fn total_singlethread_points(&self) -> u128 {
        self.info.iter().map(|x| x.1.singlethread_points).sum()
    }

    pub fn total_multithread_points(&self) -> u128 {
        self.info.iter().map(|x| x.1.multithread_points).sum()
    }

    pub fn total_points(&self) -> u128 {
        self.total_multithread_points() + self.total_singlethread_points()
    }
}
