#![no_std]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate static_assertions;

use core::convert::{TryFrom, TryInto};
use core::mem::MaybeUninit;

use cstr_core::CStr;
use num_traits::FromPrimitive;

use bad64_sys::*;

mod operand;
mod operation;
mod reg;
mod shift;
mod sysreg;

pub use operand::{Imm, Operand};
pub use operation::Operation;
pub use reg::Reg;
pub use shift::Shift;
pub use sysreg::SysReg;

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
        unsafe { CStr::from_ptr(operation_to_str(self.0.operation)) }
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

        Operand::try_from(&self.0.operands[n]).ok()
    }

    pub fn operands(&self) -> usize {
        for n in 0..MAX_OPERANDS as usize {
            if self.operand(n).is_none() {
                return n;
            }
        }

        5
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

    match r {
        0 => Ok(Instruction(unsafe { decoded.assume_init() })),
        _ => Err(DecodeError::from_i32(r).unwrap()),
    }
}

pub fn disassemble(code: &[u8], address: u64) -> impl Iterator<Item=(u64, Result<Instruction, DecodeError>)> + '_ {
    (address..).step_by(4).zip(code.chunks(4)).map(|(addr, bytes)| {
        match bytes.try_into() {
            Ok(v) => {
                let vv = u32::from_le_bytes(v);

                (addr, decode(vv, addr))
            }
            Err(_) => (addr, Err(DecodeError::EndOfInstruction))
        }
    })
}
