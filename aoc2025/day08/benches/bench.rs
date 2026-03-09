use common::read_input_u8;
use day08::*;

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
            .bench_values(|content| part1::run(&content, 1000).unwrap());
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

    #[divan::bench(name = "1_indexed")]
    fn indexed(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_indexed::run(&content).unwrap());
    }

    #[divan::bench(name = "2_vec_sort")]
    fn vec_sort(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_vec_sort::run(&content).unwrap());
    }

    #[divan::bench(name = "3_vec_indexed")]
    fn vec_indexed(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_vec_indexed::run(&content).unwrap());
    }

    #[divan::bench(name = "4_par_sort")]
    fn par_sort(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_par_sort::run(&content).unwrap());
    }
}
