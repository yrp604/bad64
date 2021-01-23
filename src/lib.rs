#![no_std]

#[macro_use]
extern crate num_derive;

use core::mem::MaybeUninit;

use cstr_core::CStr;
use num_traits::FromPrimitive;

use bad64_sys::*;

mod operand;
mod operation;

pub use operand::{Operand, OperandClass};
pub use operation::Operation;

/// Structure containing a decoded instruction
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction(bad64_sys::Instruction);

impl Instruction {
    /// Get the instruction mnemonic
    ///
    /// # Example
    /// ```
    /// use bad64::decode;
    /// let decoded = decode(0xd503201f, 0x1000).unwrap();
    /// assert_eq!(decoded.mnem(), "nop");
    // ```
    pub fn mnem(&self) -> &'static str {
        unsafe { CStr::from_ptr(get_operation(&self.0 as _)) }
            .to_str()
            .unwrap()
    }

    pub fn operation(&self) -> Operation {
        assert!(self.0.operation != 0);

        Operation::from_i32(self.0.operation).unwrap()
    }

    pub fn operand(&self, n: usize) -> Option<Operand> {
        if n >= MAX_OPERANDS as usize {
            return None;
        }

        let o = Operand::new(&self.0.operands[n]);

        if o.class() == OperandClass::None {
            return None;
        }

        Some(o)
    }

    pub fn nth(&self, n: usize) -> Operand {
        self.operand(n).unwrap()
    }
}

/// Enum for decoding errors
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, FromPrimitive)]
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

/// Decode a single instruction
///
/// # Arguments
///
/// * `ins` - A u32 of code to be decoded
/// * `address` - Location of code in memory
///
/// # Examples
/// ```
/// use bad64::decode;
/// let decoded = decode(0xd503201f, 0x1000).unwrap();
/// assert_eq!(decoded.mnem(), "nop");
/// ```
pub fn decode(ins: u32, address: u64) -> Result<Instruction, DecodeError> {
    let mut decoded = MaybeUninit::zeroed();

    let r = unsafe { aarch64_decompose(ins, decoded.as_mut_ptr(), address) };

    if r != 0 {
        Err(DecodeError::from_i32(r).unwrap())
    } else {
        Ok(Instruction(unsafe { decoded.assume_init() }))
    }
}
/*
pub fn disassemble(code: &[u8], address: u64) -> impl Iterator<Item=(u64, Result<Instruction, DecodeError>)> {
    debug_assert!(code.len() % 4 == 0);
}
*/
