use bad64::*;

#[test]
fn decode_nop() {
    let ins = decode(0xd503201f, 0x1000).unwrap();
    assert_eq!(ins.mnem(), "nop");
}

#[test]
fn decode_failure() {
    assert_eq!(decode(0x41414141, 0x1000), Err(DecodeError::Unallocated));
}
