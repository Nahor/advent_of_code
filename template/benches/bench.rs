use common::input::read_input_u8;
use template::{part1, part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

mod part1_bench {
    use super::*;

    #[divan::bench]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8(None).unwrap())
            .bench_values(|content| part1::run(&content).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8(None).unwrap())
            .bench_values(|content| part2::run(&content).unwrap());
    }
}
