use common::read_input_u8;
use day11::{
    part1, part2, part2_base_fx, part2_breadth, part2_fxhashmap, part2_inline, part2_log,
    part2_successors, part2_vec,
};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

mod part1_bench {
    use super::*;

    #[divan::bench]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part1::run(&content).unwrap());
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "0_base_fx")]
    fn base_fx(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_base_fx::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "1_log")]
    fn log(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_log::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "2_vec")]
    fn vec(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_vec::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "3_breadth")]
    fn breadth(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_breadth::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "4_inline")]
    fn inline(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_inline::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "5_successors")]
    fn successors(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_successors::run(&content, 75).unwrap());
    }

    #[divan::bench(name = "6_fxhashmap")]
    fn fxhashmap(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_fxhashmap::run(&content, 75).unwrap());
    }
}
