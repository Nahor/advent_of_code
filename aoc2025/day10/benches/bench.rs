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

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part2::run(&content).unwrap());
    }
}
