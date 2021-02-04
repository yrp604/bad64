use std::env;
use std::fs;
use std::process;

use xmas_elf::ElfFile;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        eprintln!("Usage: {} <elf>", argv[0]);
        process::exit(1);
    }

    let buf = fs::read(&argv[1]).unwrap();

    let elf = ElfFile::new(&buf).unwrap();

    let text_section = elf.find_section_by_name(".text").unwrap();

    let base = text_section.address();
    let size = text_section.size();
    let bytes = text_section.raw_data(&elf);

    println!("disassembling {} bytes from .text @ {:#x}", size, base);

    for maybe_decoded in bad64::disassemble(bytes, base) {
        match maybe_decoded {
            Ok(decoded) => {
                print!("{:04x}: ", decoded.address());
                print!("{:x?}", decoded.operation());
                for o in decoded.operands() {
                    print!(" {:x?}", o);
                }
                println!("");
            }
            Err(e) => println!("{:04x}: (bad)", e.address()),
        }
    }
}
