use bad64::*;

#[test]
fn decode_nop() {
    let ins = decode(0xd503201f, 0).unwrap();

    assert_eq!(ins.op(), Op::NOP);
    assert_eq!(ins.op().mnem(), "nop");
    assert_eq!(ins.operands().len(), 0);
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
    assert_eq!(ins.op().mnem(), "add");
    assert_eq!(ins.operands().len(), 3);

    let o0 = ins.operands()[0];
    let o1 = ins.operands()[1];
    let o2 = ins.operands()[2];
    assert_eq!(ins.operands().get(3), None);
    assert_eq!(ins.operands().get(5), None);

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

    let o0 = ins.operands().get(0);
    assert_eq!(o0, Some(&Operand::SysReg(SysReg::VBAR_EL3)));
}

#[test]
fn decode_failure() {
    assert_eq!(decode(0x41414141, 0), Err(DecodeError::Unallocated(0)));
}
