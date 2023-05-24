#![feature(test)]

extern crate test;

use {{crate_name}}::*;

#[bench]
fn rot13_basic(bench: &mut test::Bencher) {
    let source = "super secure";
    bench.iter(|| rot13(&source));
    bench.bytes = source.len() as u64;
}
