use bad64::*;

#[test]
fn decode_nop() {
    let ins = decode(0xd503201f, 0).unwrap();

    assert_eq!(ins.op(), Op::NOP);
    assert_eq!(ins.num_operands(), 0);

    assert_eq!(ins.mnem(), "nop");
}

#[test]
fn decode_iter_nop() {
    let ins1 = decode(0xd503201f, 0).unwrap();
    let mut ii = disasm(b"\x1f\x20\x03\xd5", 0);

    let ins2 = ii.next().unwrap().unwrap();

    assert_eq!(0, ins2.address());
    assert_eq!(ins1, ins2);

    assert_eq!(ii.next(), None);
}

#[test]
fn decode_iter_err() {
    let mut ii = disasm(&[0x41_u8; 8], 0);

    assert_eq!(ii.next().unwrap(), Err(DecodeError::Unallocated(0)));
    assert_eq!(ii.next().unwrap(), Err(DecodeError::Unallocated(4)));
    assert_eq!(ii.next(), None);
}

#[test]
fn decode_iter_short() {
    let mut ii = disasm(&[0x41_u8; 3], 0);

    assert_eq!(ii.next().unwrap(), Err(DecodeError::Short(0)));
    assert_eq!(ii.next(), None);
}

#[test]
fn decode_add() {
    // add x0, x1, #0x41
    let ins = decode(0x91010420, 0).unwrap();

    assert_eq!(ins.op(), Op::ADD);
    assert_eq!(ins.num_operands(), 3);
    assert_eq!(ins.mnem(), "add");

    let o0 = ins.operand(0).unwrap();
    let o1 = ins.operand(1).unwrap();
    let o2 = ins.operand(2).unwrap();
    assert_eq!(ins.operand(3), None);
    assert_eq!(ins.operand(5), None);

    assert_eq!(
        o0,
        Operand::Reg {
            reg: Reg::X0,
            arrspec: None,
        }
    );
    assert_eq!(
        o1,
        Operand::Reg {
            reg: Reg::X1,
            arrspec: None,
        }
    );
    assert_eq!(
        o2,
        Operand::Imm64 {
            imm: Imm {
                neg: false,
                val: 0x41
            },
            shift: None
        }
    );
}

#[test]
fn system_reg() {
    // msr vbar_el3, x0
    let ins = decode(0xd51ec000, 0).unwrap();

    let o0 = ins.operand(0).unwrap();
    assert_eq!(o0, Operand::SysReg(SysReg::VBAR_EL3));
}

#[test]
fn decode_failure() {
    assert_eq!(decode(0x41414141, 0), Err(DecodeError::Unallocated(0)));
}
