#![no_std]

use core::mem::MaybeUninit;

use cstr_core::CStr;

use bad64_sys::*;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[repr(i32)]
pub enum DecodeError {
    Reserved = DECODE_STATUS_RESERVED,
    Unmatched = DECODE_STATUS_UNMATCHED,
    Unallocated = DECODE_STATUS_UNALLOCATED,
    Undefined = DECODE_STATUS_UNDEFINED,
    EndOfInstruction = DECODE_STATUS_END_OF_INSTRUCTION,
    Lost = DECODE_STATUS_LOST,
    Unreachable = DECODE_STATUS_UNREACHABLE,
}

impl From<i32> for DecodeError {
    fn from(v: i32) -> Self {
        match v {
            DECODE_STATUS_RESERVED => Self::Reserved,
            DECODE_STATUS_UNMATCHED => Self::Unmatched,
            DECODE_STATUS_UNALLOCATED => Self::Unallocated,
            DECODE_STATUS_UNDEFINED => Self::Undefined,
            DECODE_STATUS_END_OF_INSTRUCTION => Self::EndOfInstruction,
            DECODE_STATUS_LOST => Self::Lost,
            DECODE_STATUS_UNREACHABLE => Self::Unreachable,
            _ => panic!("unknown decode status"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction(bad64_sys::Instruction);

impl Instruction {
    pub fn mnem(&self) -> &str {
        unsafe { CStr::from_ptr(get_operation(&self.0 as _)) }
            .to_str()
            .unwrap()
    }
}

pub fn decode(ins: u32, address: u64) -> Result<Instruction, DecodeError> {
    let mut decoded = MaybeUninit::uninit();

    let r = unsafe { aarch64_decompose(ins, decoded.as_mut_ptr(), address) };

    if r != 0 {
        Err(DecodeError::from(r))
    } else {
        Ok(Instruction(unsafe { decoded.assume_init() }))
    }
}
