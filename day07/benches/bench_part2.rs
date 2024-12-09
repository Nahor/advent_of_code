use common::input::read_input_u8;
use day07::part2;

fn main() {
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_u8(None).unwrap())
        .bench_values(|content| part2::run(&content).unwrap());
}
