use common::read_input_u8;
use day20::*;

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
            .bench_values(|content| part1::run(&content, 100).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2::run(&content, 100).unwrap());
    }

    #[divan::bench(name = "1_rayon")]
    fn rayon(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_rayon::run(&content, 100).unwrap());
    }

    #[divan::bench(name = "2_cost_first")]
    fn cost_first(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_cost_first::run(&content, 100).unwrap());
    }

    #[divan::bench(name = "3_combination")]
    fn combination(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_combination::run(&content, 100).unwrap());
    }

    #[divan::bench(name = "4_filter_map")]
    fn filter_map(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_filter_map::run(&content, 100).unwrap());
    }
}
