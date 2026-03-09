use common::read_input_u8;
use day04::*;

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

    #[divan::bench(name = "1_rayon")]
    fn rayon(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part1_rayon::run(&content).unwrap());
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

    #[divan::bench(name = "1_floodfill")]
    fn floodfill(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_floodfill::run(&content).unwrap());
    }
}
