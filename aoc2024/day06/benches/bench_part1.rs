use common::read_input_u8;
use day06::part1;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_u8!(None).unwrap())
        .bench_values(|content| part1::run(&content).unwrap());
}
