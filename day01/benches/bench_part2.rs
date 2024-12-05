use common::input::read_input_str;
use day01::part2;

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
fn process_map(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run_map(&content).unwrap());
}

#[divan::bench]
fn process_vecmap(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run_vecmap(&content).unwrap());
}

#[divan::bench]
fn process_sorted(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run_sorted(&content).unwrap());
}

#[divan::bench]
fn process_vec_sorted(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run_vec_sorted(&content).unwrap());
}

#[divan::bench]
fn process_vec_sorted2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run_vec_sorted2(&content).unwrap());
}
