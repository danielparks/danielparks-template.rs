#![feature(test)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::let_underscore_untyped,
    clippy::manual_string_new,
    clippy::map_unwrap_or
)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

extern crate test;

use {{crate_name}}::*;

#[bench]
fn rot13_basic(bench: &mut test::Bencher) {
    let source = "super secure";
    bench.iter(|| rot13(source));
    bench.bytes = source.len() as u64;
}
