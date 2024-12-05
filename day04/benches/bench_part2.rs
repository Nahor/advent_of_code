use common::input::{read_input_str, read_input_u8};
use day04::{part2, part2_u8, part2_u8_linear};

fn main() {
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run(&content).unwrap());
}

#[divan::bench]
fn process_u8(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_u8(None).unwrap())
        .bench_values(|content| part2_u8::run(&content).unwrap());
}

#[divan::bench]
fn process_u8_linear(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_u8(None).unwrap())
        .bench_values(|content| part2_u8_linear::run(&content).unwrap());
}
