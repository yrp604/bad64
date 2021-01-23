use bad64::*;

#[test]
fn decode_nop() {
    let ins = decode(0xd503201f, 0).unwrap();
    assert_eq!(ins.mnem(), "nop");
}

#[test]
fn decode_add() {
    // add x0, x1, #0x41
    let ins = decode(0x91010420, 0).unwrap();

    assert_eq!(ins.operation(), Operation::ADD);

    let o0 = ins.operand(0).unwrap();
    let o1 = ins.operand(1).unwrap();
    let o2 = ins.operand(2).unwrap();
    assert_eq!(ins.operand(3), None);
    assert_eq!(ins.operand(5), None);

    assert_eq!(o0.class(), OperandClass::Reg);
    assert_eq!(o1.class(), OperandClass::Reg);
    assert_eq!(o2.class(), OperandClass::Imm64);

    let r0 = o0.reg(0).unwrap();
    let r1 = o1.reg(0).unwrap();
    assert_eq!(o0.reg(1), None);
    assert_eq!(o0.reg(5), None);

    assert_eq!(r0.name(), "x0");
    assert_eq!(r1.name(), "x1");

    assert_eq!(o2.imm().unwrap(), 0x41);
}

#[test]
fn system_reg() {
    // msr vbar_el3, x0
    let ins = decode(0xd51ec000, 0).unwrap();

    let o0 = ins.operand(0).unwrap();
    assert_eq!(o0.class(), OperandClass::SysReg);

    let sr = o0.sysreg().unwrap();

    assert_eq!(sr.name(), "vbar_el3");
}

#[test]
fn decode_failure() {
    assert_eq!(decode(0x41414141, 0), Err(DecodeError::Unallocated));
}
