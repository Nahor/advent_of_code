use common::input::read_input_str;
use template::part2;

fn main() {
    divan::main();
}

#[divan::bench]
fn process(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| read_input_str(None).unwrap())
        .bench_values(|content| part2::run(&content).unwrap());
}
