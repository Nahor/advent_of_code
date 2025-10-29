use common::read_input_str;
use day02::{
    part2_brute_force, part2_optimized, part2_optimized2, part2_optimized2_dir,
    part2_optimized2_skip, part2_optimized2_skip_more, part2_optimized2_tuple,
};

fn main() {
    divan::main();
}

#[divan::bench]
fn process_brute(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_brute_force::run(&content).unwrap());
}

#[divan::bench]
fn process_optimized(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_optimized::run(&content).unwrap());
}

#[divan::bench]
fn process_optimized2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_optimized2::run(&content).unwrap());
}

#[divan::bench]
fn process_optimized2_dir(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_optimized2_dir::run(&content).unwrap());
}

#[divan::bench]
fn process_optimized2_tuple(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_optimized2_tuple::run(&content).unwrap());
}

#[divan::bench]
fn process_optimized2_skip(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_optimized2_skip::run(&content).unwrap());
}

#[divan::bench]
fn process_optimized2_skip_more(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str!(None).unwrap())
        .bench_values(|content| part2_optimized2_skip_more::run(&content).unwrap());
}
