#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::let_underscore_untyped,
    clippy::manual_string_new,
    clippy::map_unwrap_or,
    clippy::module_name_repetitions
)]
// Other restriction lints
#![warn(clippy::arithmetic_side_effects)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
#[allow(clippy::wildcard_imports)]
use {{crate_name}}::*;
use std::convert::TryInto;
use std::time::Duration;

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("general");
    group
        .noise_threshold(0.10)
        .significance_level(0.01)
        .confidence_level(0.99)
        .sample_size(300)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10));

    for input in ["", "super secure", "super long and super secure"] {
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("rot13", input.len()),
            input,
            |b, input| b.iter(|| rot13(input)),
        );
    }

    group.finish();
}

criterion_group!(general_group, benchmarks);
criterion_main!(general_group);
