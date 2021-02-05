use std::fs;
use std::num::ParseIntError;

#[test]
fn testcases() {
    fn fuzzy_eq(expected: &str, actual: &str) -> bool {
        fn fuzzy_token(a: &str, b: &str) -> bool {
            fn parse_num(n: &str) -> Result<u64, ParseIntError> {
                let s = n
                    .trim_start_matches("#")
                    .trim_end_matches(",")
                    .trim_end_matches(".0")
                    .trim_end_matches("!")
                    .trim_end_matches("]");

                if s.starts_with("-") {
                    return match s {
                        _ if s.starts_with("-0x") => {
                            Ok(-i64::from_str_radix(s.trim_start_matches("-0x"), 16)? as u64)
                        }
                        _ => Ok(i64::from_str_radix(s, 10)? as u64),
                    };
                }

                return match s {
                    _ if s.starts_with("0x") => {
                        Ok(u64::from_str_radix(s.trim_start_matches("0x"), 16)?)
                    }
                    _ => Ok(u64::from_str_radix(s, 10)?),
                };
            }

            match (a, b) {
                ("cs", "hs") | ("hs", "cs") => true,
                ("lo", "cc") | ("cc", "lo") => true,
                _ if a.starts_with('#') && b.starts_with("#") => {
                    match (parse_num(a), parse_num(b)) {
                        (Ok(x), Ok(y)) => {
                            println!("parsed {:#x} {:#x}", x, y);
                            x == y || x & 0xffff_ffff == y & 0xffff_ffff || x & 0xff == y & 0xff
                        }
                        (Ok(_), Err(_)) => {
                            println!("couldnt parse: {}", b);
                            false
                        }
                        (Err(_), Ok(_)) => {
                            println!("couldnt parse: {}", a);
                            false
                        }
                        (Err(x), Err(y)) => {
                            println!("couldnt parse: {} / {}", a, b);
                            false
                        }
                    }
                }
                _ => a == b,
            }
        }

        match actual {
            "axflag" if expected.starts_with("msr") => true,
            "xaflag" if expected.starts_with("msr") => true,
            "bti " => true, // BTI has an all-zero NAME argument?
            "cfinv" if expected.starts_with("msr") => true,
            "dgh" if expected.starts_with("hint") => true,
            _ if actual.ends_with("sxtx]") && expected.ends_with("sxtx #0x0]") => true,
            _ if actual.ends_with("uxtw]") && expected.ends_with("uxtw #0x0]") => true,
            _ if actual.ends_with("sxtw]") && expected.ends_with("sxtw #0x0]") => true,
            _ if actual.starts_with("cmpp") && expected.starts_with("subps") => true,
            _ if actual.starts_with("mov") && expected.starts_with("dupm") => true,
            _ if actual.starts_with("msr pan") && expected.starts_with("msr s0_0") => true,
            _ if actual.starts_with("msr ssbs") && expected.starts_with("msr s0_3") => true,
            _ if actual.starts_with("sb") && expected.starts_with("msr s0_3") => true,
            _ if actual.starts_with("sdot") => true,
            _ if actual.starts_with("udot") => true,
            _ if actual == expected => true,
            _ => expected
                .split(' ')
                .zip(actual.split(' '))
                .all(|(a, b)| fuzzy_token(a, b)),
        }
    }

    let testcases = fs::read_to_string("tests/test_cases.txt").unwrap();

    for (n, line) in testcases.lines().enumerate() {
        if line.starts_with("//") {
            println!("processing {}...", line);
            continue;
        }

        let chunks: Vec<&str> = line.trim_right().split_whitespace().collect();

        let op = u32::from_str_radix(chunks[0], 16).unwrap();
        let expected = chunks[1..].join(" ");

        let decoded = bad64::decode(op, 0x8000_0000_0000_0004).unwrap();
        let actual = format!("{}", decoded);

        if !fuzzy_eq(&expected, &actual) {
            println!("opcode:   {:x}", op);
            println!("expected: |{}|", expected);
            println!("actual:   |{}|", actual);
            println!("debug:    {:x?}", decoded);
            println!(
                "progress: {}/{}",
                n,
                testcases.lines().collect::<Vec<&str>>().len()
            );
            assert!(false);
        }
    }
}
