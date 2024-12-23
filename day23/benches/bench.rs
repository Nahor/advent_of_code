use common::read_input_u8;
use day23::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

mod part1_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part1::run(&content).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench(name = "0_base", sample_count = 1, ignore)]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2::run(&content).unwrap());
    }

    #[divan::bench(name = "1_optimize", sample_count = 1, ignore)]
    fn optimize(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_optimize::run(&content).unwrap());
    }

    #[divan::bench(name = "2_brute_force")]
    fn brute_force(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_brute_force::run(&content).unwrap());
    }
}
