use common::read_input_u8;
use day09::*;

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

    #[divan::bench(name = "1_i64")]
    fn i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part1_i64::run(&content).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2::run(&content).unwrap());
    }

    #[divan::bench(name = "1_sort")]
    fn sort(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_sort::run(&content).unwrap());
    }

    #[divan::bench(name = "2_bin_heap")]
    fn bin_heap(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_bin_heap::run(&content).unwrap());
    }

    #[divan::bench(name = "3_rayon")]
    fn rayon(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_rayon::run(&content).unwrap());
    }
}
