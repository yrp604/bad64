use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <hex u32>", args[0]);
        process::exit(1);
    }

    let op = u32::from_str_radix(args[1].trim_start_matches("0x"), 16)
        .expect(&format!("Could not parse {} as hex u32", args[1]));

    let base = match args.get(2) {
        Some(arg) => u64::from_str_radix(arg.trim_start_matches("0x"), 16)
            .expect(&format!("Could not parse {} as hex u64", arg)),
        None => 0x1000,
    };

    let decoded = bad64::decode(op, base).expect(&format!("Could not decode {:#x}", op));

    println!("{:#x?}", decoded);
    println!("{}", decoded);
}
