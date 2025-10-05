use common::read_input_u8;
use day18::*;

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
            .bench_values(|content| part1::run(&content, 70, 1024).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2::run(&content, 70, 1024).unwrap());
    }

    // This is extremely slow compared to the others, so, if not ignored, do it
    // just once to get an order of magnitude rather than a precise amount
    #[divan::bench(name = "1_brute_force", sample_count = 1, ignore)]
    fn brute_force(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_brute_force::run(&content, 70, 1024).unwrap());
    }

    #[divan::bench(name = "2_binary_search")]
    fn binary_search(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_binary_search::run(&content, 70, 1024).unwrap());
    }

    #[divan::bench(name = "3_rayon")]
    fn rayon(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_rayon::run(&content, 70, 1024).unwrap());
    }

    #[divan::bench(name = "4_simplify")]
    fn simplify(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_simplify::run(&content, 70, 1024).unwrap());
    }
}
