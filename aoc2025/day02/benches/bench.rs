use common::read_input_str;
use day02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

mod part1_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_str!(None).unwrap())
            .bench_values(|content| part1::run(&content).unwrap());
    }

    #[divan::bench(name = "1_chrisbiscardi_ref")]
    fn chrisbiscardi_ref(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_str!(None).unwrap())
            .bench_values(|content| part1_chrisbiscardi_ref::run(&content).unwrap());
    }

    #[divan::bench(name = "2_chrisbiscardi_skip")]
    fn chrisbiscardi_skip(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_str!(None).unwrap())
            .bench_values(|content| part1_chrisbiscardi_skip::run(&content).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_str!(None).unwrap())
            .bench_values(|content| part2::run(&content).unwrap());
    }
}
