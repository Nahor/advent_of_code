pub fn run() {
    // Hardcoded Rust-version of the part 1 program, ran with a bunch of
    // different values for register A t(28066687 being the original value from
    // AoC) that give the same output
    const EXPECTED: usize = 730571405;
    for mut a in [
        28057973, 28058485, 28066687, 28058495, 28058487, 28058613, 28058621, 28066167, 28066165,
        28066679, 28057973, 28066677, 28057975, 28066685, 28058101, 28058493,
    ] {
        let mut b;
        let mut c;
        let mut result = 0;
        println!("a: {a:x}");
        loop {
            b = a % 8;
            b ^= 1;
            c = a >> b;
            b ^= c;
            b ^= 4;

            println!(
                "a:...{:x} ({a:x}), out: {}",
                a & ((1 << 10) - 1),
                (((b % 8) as u8) + b'0') as char
            );

            a >>= 3;
            result = result * 10 + b % 8;
            if (a == 0) || result > EXPECTED {
                break;
            }
        }
        println!(
            "{result:?} => {} (expected {EXPECTED})",
            if result == EXPECTED { "OK" } else { "BAD" }
        );
    }
}
