use common::read_input_u8;
use day10::*;

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

    #[divan::bench_group(name = "0_base_parts")]
    mod base_parts {
        use super::*;

        #[divan::bench(name = "0_parse")]
        fn parse(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| parse::int::parse((&content[..]).into()).unwrap());
        }

        #[divan::bench(name = "1_process")]
        fn process(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| {
                    let content = read_input_u8!(None).unwrap();
                    parse::int::parse((&content[..]).into()).unwrap()
                })
                .bench_values(|content| part1::process(&content).unwrap());
        }
    }

    #[divan::bench(name = "1_bitvec")]
    fn bitvec(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part1_bitvec::run(&content).unwrap());
    }

    #[divan::bench_group(name = "1_bitvec_parts")]
    mod bitvec_parts {
        use super::*;

        #[divan::bench(name = "0_parse")]
        fn parse(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| parse::bitvec::parse((&content[..]).into()).unwrap());
        }

        #[divan::bench(name = "1_process")]
        fn process(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| {
                    let content = read_input_u8!(None).unwrap();
                    parse::bitvec::parse((&content[..]).into()).unwrap()
                })
                .bench_values(|content| part1_bitvec::process(&content).unwrap());
        }
    }
}

mod part2_bench {
    use super::*;

    #[divan::bench_group(name = "0_z3")]
    mod z3 {
        use super::*;

        // Too slow => ignore (~1min)
        #[divan::bench(name = "0_base", sample_count = 1, sample_size = 1, ignore)]
        fn base(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| part2_z3_optimize::run(&content).unwrap());
        }

        // Case with lots of solutions (4433). This wil be used to check speed improvements
        #[divan::bench(name = "1_machine_4433", sample_count = 1, sample_size = 1)]
        fn machine_4433(bencher: divan::Bencher) {
            bencher
            .with_inputs(|| b"[.##..###] (2) (2,4,5,6,7) (2,5,7) (1) (6) (1,4,6) (0,2,3,4,5) (4,6) (0,2,3,5,6,7) (0,3,5,7) {41,21,54,41,41,67,52,51}")
            .bench_values(|content| part2_z3::run(content).unwrap());
        }

        // Case with lots of solutions (20827). This wil be used to check speed improvements
        #[divan::bench(name = "1_machine_20827", sample_count = 1, sample_size = 1)]
        fn machine_20827(bencher: divan::Bencher) {
            bencher
            .with_inputs(|| b"[#..#.#] (0,1) (0,1,4) (2) (0,1,2) (4,5) (1,2,3,4) (0,1,2,5) (4) {61,67,49,6,48,23}")
            .bench_values(|content| part2_z3::run(content).unwrap());
        }

        // Case with lots of solutions (20827). This wil be used to check speed improvements
        #[divan::bench(name = "2_optimize")]
        fn z3_optimize(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| part2_z3_optimize::run(&content).unwrap());
        }

        // Case with lots of solutions (4433). This wil be used to check speed improvements
        #[divan::bench(name = "3_optimize_4433", sample_count = 1, sample_size = 1)]
        fn optimize_4433(bencher: divan::Bencher) {
            bencher
            .with_inputs(|| b"[.##..###] (2) (2,4,5,6,7) (2,5,7) (1) (6) (1,4,6) (0,2,3,4,5) (4,6) (0,2,3,5,6,7) (0,3,5,7) {41,21,54,41,41,67,52,51}")
            .bench_values(|content| part2_z3_optimize::run(content).unwrap());
        }

        // Case with lots of solutions (20827). This wil be used to check speed improvements
        #[divan::bench(name = "3_optimize_20827", sample_count = 1, sample_size = 1)]
        fn optimize_20827(bencher: divan::Bencher) {
            bencher
            .with_inputs(|| b"[#..#.#] (0,1) (0,1,4) (2) (0,1,2) (4,5) (1,2,3,4) (0,1,2,5) (4) {61,67,49,6,48,23}")
            .bench_values(|content| part2_z3_optimize::run(content).unwrap());
        }
    }

    #[divan::bench_group(name = "1_good_lp")]
    mod good_lp {
        use super::*;

        #[divan::bench(name = "0_base")]
        fn base(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| part2_good_lp::run(&content).unwrap());
        }

        // Case with lots of solutions (4433). To compare with Z3
        #[divan::bench(name = "1_machine_4433")]
        fn machine_4433(bencher: divan::Bencher) {
            bencher
            .with_inputs(|| b"[.##..###] (2) (2,4,5,6,7) (2,5,7) (1) (6) (1,4,6) (0,2,3,4,5) (4,6) (0,2,3,5,6,7) (0,3,5,7) {41,21,54,41,41,67,52,51}")
            .bench_values(|content| part2_good_lp::run(content).unwrap());
        }

        // Case with lots of solutions (20827). To compare with Z3
        #[divan::bench(name = "1_machine_20827")]
        fn machine_20827(bencher: divan::Bencher) {
            bencher
            .with_inputs(|| b"[#..#.#] (0,1) (0,1,4) (2) (0,1,2) (4,5) (1,2,3,4) (0,1,2,5) (4) {61,67,49,6,48,23}")
            .bench_values(|content| part2_good_lp::run(content).unwrap());
        }
    }

    #[divan::bench(name = "2_smart", ignore)]
    fn smart(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2_smart::run(&content).unwrap());
    }
}
