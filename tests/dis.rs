use bad64::*;

#[test]
fn decode_nop() {
    let ins = decode(0xd503201f, 0).unwrap();

    assert_eq!(ins.operation(), Operation::NOP);
    assert_eq!(ins.operands(), 0);

    assert_eq!(ins.mnem(), "nop");
}

#[test]
fn decode_add() {
    // add x0, x1, #0x41
    let ins = decode(0x91010420, 0).unwrap();

    assert_eq!(ins.operation(), Operation::ADD);
    assert_eq!(ins.operands(), 3);

    let o0 = ins.operand(0).unwrap();
    let o1 = ins.operand(1).unwrap();
    let o2 = ins.operand(2).unwrap();
    assert_eq!(ins.operand(3), None);
    assert_eq!(ins.operand(5), None);

    assert_eq!(o0, Operand::Reg(Reg::X0));
    assert_eq!(o1, Operand::Reg(Reg::X1));
    assert_eq!(o2, Operand::Imm64(0x41));
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
    assert_eq!(decode(0x41414141, 0), Err(DecodeError::Unallocated));
}
