use common::input::read_input_u8;
use day10::{part1, part1_out_param};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_u8(None).unwrap())
        .bench_values(|content| part1::run(&content).unwrap());
}

#[divan::bench]
fn process_out_param(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_u8(None).unwrap())
        .bench_values(|content| part1_out_param::run(&content).unwrap());
}
