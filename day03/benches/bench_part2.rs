use day03::{input::read_input, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input(None).unwrap())
        .bench_values(|content| part2::run(&content).unwrap());
}

#[divan::bench]
fn process_winnow(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input(None).unwrap())
        .bench_values(|content| part2::run_winnow(&content).unwrap());
}

#[divan::bench]
fn process_winnow_skip(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input(None).unwrap())
        .bench_values(|content| part2::run_winnow_skip(&content).unwrap());
}

#[divan::bench]
fn process_winnow_compute(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input(None).unwrap())
        .bench_values(|content| part2::run_winnow_compute(&content).unwrap());
}
