# Binja Arm64 Disassembler

[![Build Status]][actions] [![Latest Version]][crates.io] [![Latest Docs]][docs.rs]

[Build Status]: https://img.shields.io/github/workflow/status/yrp604/bad64/Rust
[actions]: https://github.com/yrp604/bad64/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/bad64.svg
[crates.io]: https://crates.io/crates/bad64
[Latest Docs]: https://docs.rs/bad64/badge.svg
[docs.rs]: https://docs.rs/bad64

These are bindings to the [Binary Ninja](https://binary.ninja) arm64
[architecture/disassembler plugin](https://github.com/Vector35/arch-arm64).

Note that while Binary Ninja is an (excellent) commercial product, the
disassembler is entirely Apache 2 licensed and may be used without a license.
To install, just add bad64 as a normal dependency in Cargo.toml.

For more information on how this disassembler was created, see [this blogpost][blogpost]
by [Andrew Lamoureux][andrew].

[blogpost]: https://binary.ninja/2021/04/05/groundup-aarch64.html
[andrew]: https://github.com/lwerdna

For docs and usage, please see [docs.rs](http://docs.rs/bad64) and the
[examples](examples).

```
$ cargo run --example decode 0x91010420
Instruction {
    address: 0x1000,
    opcode: 0x91010420,
    op: ADD,
    num_operands: 0x3,
    operands: [
        Reg {
            reg: X0,
            arrspec: None,
        },
        Reg {
            reg: X1,
            arrspec: None,
        },
        Imm64 {
            imm: Unsigned(
                0x41,
            ),
            shift: None,
        },
    ],
}
add x0, x1, #0x41
```
