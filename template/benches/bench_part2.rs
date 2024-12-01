use template::{input::read_input, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input(None).unwrap())
        .bench_values(|content| part2::run(&content).unwrap());
}
