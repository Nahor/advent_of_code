use common::read_input_u8;
use day05::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

mod parse_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| parse::parse(&content).unwrap());
    }

    #[divan::bench(name = "1_no_id")]
    fn no_id(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| parse::parse_no_id(&content).unwrap());
    }

    #[divan::bench(name = "2_custom")]
    fn custom(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| parse::custom::parse(&content).unwrap());
    }
}

mod part1_bench {
    use super::*;

    #[divan::bench(name = "0_base")]
    fn base(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| read_input_u8!(None).unwrap())
            .bench_values(|content| part1::run(&content).unwrap());
    }

    #[divan::bench(name = "0_no_parse")]
    fn no_parse(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| parse::parse(&read_input_u8!(None).unwrap()).unwrap())
            .bench_values(|content| part1::process(content).unwrap());
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

    #[divan::bench_group(name = "1_sort")]
    mod sort {
        use super::*;

        #[divan::bench(name = "0_base")]
        fn sort(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| part2_sort::run(&content).unwrap());
        }

        #[divan::bench(name = "1_no_parse")]
        fn sort_no_parse(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| parse::parse(&read_input_u8!(None).unwrap()).unwrap())
                .bench_values(|content| part2_sort::process(content).unwrap());
        }
    }

    #[divan::bench_group(name = "2_custom")]
    mod custom {
        use super::*;

        #[divan::bench(name = "0_parse")]
        fn parse(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| parse::custom::parse(&content).unwrap());
        }

        #[divan::bench(name = "1_process")]
        fn process(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| parse::custom::parse(&read_input_u8!(None).unwrap()).unwrap())
                .bench_values(|content| part2_custom::process(content).unwrap());
        }

        #[divan::bench(name = "2_total")]
        fn total(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| read_input_u8!(None).unwrap())
                .bench_values(|content| part2_custom::run(&content).unwrap());
        }
    }
}
