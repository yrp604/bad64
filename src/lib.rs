//! # bad64
//!
//! bad64 is a set of Rust bindings to the Binja Arm64 Disassembler.
//!
//! For more information about the disassembler, please see the
//! [upstream](https://github.com/Vector35/arch-arm64/tree/dev/disassembler)
//! repo.
//!
//! There are two main entry points:
//! 1. [`decode`] for decoding a single instruction.
//! ```
//! use bad64::{decode, Op};
//! // nop - "\x1f\x20\x03\xd5"
//! let decoded = decode(0xd503201f, 0x1000).unwrap();
//!
//! assert_eq!(decoded.address(), 0x1000);
//! assert_eq!(decoded.operands().len(), 0);
//! assert_eq!(decoded.op(), Op::NOP);
//! assert_eq!(decoded.op().mnem(), "nop");
//! ```
//!
//! 2. [`disasm`] for disassembling a byte sequence.
//! ```
//! use bad64::{disasm, Op, Operand, Reg, Imm};
//!
//! // 1000: str   x0, [sp, #-16]! ; "\xe0\x0f\x1f\xf8"
//! // 1004: ldr   x0, [sp], #16   ; "\xe0\x07\x41\xf8"
//! let mut decoded_iter = disasm(b"\xe0\x0f\x1f\xf8\xe0\x07\x41\xf8", 0x1000);
//!
//! let push = decoded_iter.next().unwrap().unwrap();
//!
//! // check out the push
//! assert_eq!(push.address(), 0x1000);
//! assert_eq!(push.operands().len(), 2);
//! assert_eq!(push.op(), Op::STR);
//! assert_eq!(
//!     push.operands()[0],
//!     Operand::Reg { reg: Reg::X0, arrspec: None }
//! );
//! assert_eq!(
//!     push.operands()[1],
//!     Operand::MemPreIdx { reg: Reg::SP, imm: Imm::Signed(-16) }
//! );
//! assert_eq!(push.operands().get(2), None);
//!
//! let pop = decoded_iter.next().unwrap().unwrap();
//!
//! // check out the pop
//! assert_eq!(pop.address(), 0x1004);
//! assert_eq!(pop.operands().len(), 2);
//! assert_eq!(pop.op(), Op::LDR);
//! assert_eq!(
//!     pop.operands().get(0),
//!     Some(&Operand::Reg { reg: Reg::X0, arrspec: None })
//! );
//! assert_eq!(
//!     pop.operands().get(1),
//!     Some(&Operand::MemPostIdxImm { reg: Reg::SP, imm: Imm::Signed(16) })
//! );
//! assert_eq!(pop.operands().get(2), None);
//!
//! // make sure there's nothing left
//! assert_eq!(decoded_iter.next(), None);
//! ```

#![no_std]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate static_assertions;

#[cfg(feature = "std")]
extern crate std;

use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::hash::{Hash, Hasher};

use num_traits::FromPrimitive;

use bad64_sys::*;

mod arrspec;
mod condition;
mod op;
mod operand;
mod reg;
mod shift;
mod sysreg;

pub use arrspec::ArrSpec;
pub use condition::Condition;
pub use op::Op;
pub use operand::{Imm, Operand};
pub use reg::Reg;
pub use shift::Shift;
pub use sysreg::SysReg;

/// A decoded instruction
#[derive(Clone)]
pub struct Instruction {
    address: u64,
    opcode: u32,
    op: Op,
    num_operands: usize,
    operands: [Operand; MAX_OPERANDS as usize],
}

// Needed because MaybeUninit doesn't allow derives
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.address() == other.address()
            && self.op() == other.op()
            && self.opcode() == other.opcode()
            && self.num_operands == other.num_operands
            && self
                .operands()
                .iter()
                .zip(other.operands().iter())
                .all(|(a, b)| a == b)
    }
}

impl Eq for Instruction {}

impl Hash for Instruction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
        self.opcode.hash(state);
        self.op.hash(state);
        self.num_operands.hash(state);

        for o in self.operands() {
            o.hash(state);
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.op())?;

        let mut ops = self.operands().iter();

        if let Some(op) = ops.next() {
            write!(f, " {}", op)?;

            for op in ops {
                write!(f, ", {}", op)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("address", &self.address)
            .field("opcode", &self.opcode)
            .field("op", &self.op)
            .field("num_operands", &self.num_operands)
            .field("operands", &self.operands())
            .finish()
    }
}

impl Instruction {
    /// Returns the instruction address
    ///
    /// # Example
    /// ```
    /// use bad64::decode;
    /// // nop - "\x1f\x20\x03\xd4"
    /// let decoded = decode(0xd503201f, 0x1000).unwrap();
    /// assert_eq!(decoded.address(), 0x1000);
    /// ```
    pub fn address(&self) -> u64 {
        self.address
    }

    /// Returns the instruction opcode
    ///
    /// # Example
    /// ```
    /// use bad64::decode;
    /// // nop - "\x1f\x20\x03\xd4"
    /// let decoded = decode(0xd503201f, 0x1000).unwrap();
    /// assert_eq!(decoded.opcode(), 0xd503201f);
    /// ```
    pub fn opcode(&self) -> u32 {
        self.opcode
    }

    /// Returns the instruction operation
    ///
    /// # Example
    /// ```
    /// use bad64::{decode, Op};
    /// // nop - "\x1f\x20\x03\xd4"
    /// let decoded = decode(0xd503201f, 0x1000).unwrap();
    /// assert_eq!(decoded.op(), Op::NOP);
    // ```
    pub fn op(&self) -> Op {
        self.op
    }

    /// Returns a slice of Operands
    ///
    /// # Example
    /// ```
    /// use bad64::{decode, Operand, Reg};
    ///
    /// // eor x0, x1, x2  - "\x20\x00\x02\xca"
    /// let decoded = decode(0xca020020, 0x1000).unwrap();
    ///
    /// let mut ops = decoded.operands();
    ///
    /// assert_eq!(ops.len(), 3);
    /// assert_eq!(ops[0], Operand::Reg { reg: Reg::X0, arrspec: None });
    /// assert_eq!(ops[1], Operand::Reg { reg: Reg::X1, arrspec: None });
    /// assert_eq!(ops[2], Operand::Reg { reg: Reg::X2, arrspec: None });
    /// ```
    pub fn operands(&self) -> &[Operand] {
        &self.operands[..self.num_operands]
    }
}
/// Decoding errors types
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[repr(i32)]
pub enum DecodeError {
    Reserved(u64),
    Unmatched(u64),
    Unallocated(u64),
    Undefined(u64),
    EndOfInstruction(u64),
    Lost(u64),
    Unreachable(u64),
    Short(u64),
    ErrorOperands(u64),
}

impl DecodeError {
    fn new(code: i32, address: u64) -> Self {
        match code {
            DECODE_STATUS_RESERVED => Self::Reserved(address),
            DECODE_STATUS_UNMATCHED => Self::Unmatched(address),
            DECODE_STATUS_UNALLOCATED => Self::Unallocated(address),
            DECODE_STATUS_UNDEFINED => Self::Undefined(address),
            DECODE_STATUS_END_OF_INSTRUCTION => Self::EndOfInstruction(address),
            DECODE_STATUS_LOST => Self::Lost(address),
            DECODE_STATUS_UNREACHABLE => Self::Unreachable(address),
            DECODE_STATUS_ERROR_OPERANDS => Self::ErrorOperands(address),
            _ => panic!("unknown decode error code"),
        }
    }

    pub fn address(&self) -> u64 {
        match self {
            Self::Reserved(a) => *a,
            Self::Unmatched(a) => *a,
            Self::Unallocated(a) => *a,
            Self::Undefined(a) => *a,
            Self::EndOfInstruction(a) => *a,
            Self::Lost(a) => *a,
            Self::Unreachable(a) => *a,
            Self::Short(a) => *a,
            Self::ErrorOperands(a) => *a,
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::Reserved(x) => write!(f, "Reserved: {:#x}", x),
            DecodeError::Unmatched(x) => write!(f, "Unmatched: {:#x}", x),
            DecodeError::Unallocated(x) => write!(f, "Unallocated: {:#x}", x),
            DecodeError::Undefined(x) => write!(f, "Undefined: {:#x}", x),
            DecodeError::EndOfInstruction(x) => write!(f, "EndOfInstruction: {:#x}", x),
            DecodeError::Lost(x) => write!(f, "Lost: {:#x}", x),
            DecodeError::Unreachable(x) => write!(f, "Unreachable: {:#x}", x),
            DecodeError::Short(x) => write!(f, "Short: {:#x}", x),
            DecodeError::ErrorOperands(x) => write!(f, "ErrorOperands: {:#x}", x),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// Decode a single instruction
///
/// # Arguments
///
/// * `ins` - A little endian u32 of code to be decoded
/// * `address` - Location of code in memory
///
/// # Examples
/// ```
/// use bad64::{decode, Op};
///
/// // NOTE: little endian
/// let decoded = decode(0xd503201f, 0x1000).unwrap();
///
/// assert_eq!(decoded.operands().len(), 0);
/// assert_eq!(decoded.operands(), &[]);
/// assert_eq!(decoded.op(), Op::NOP);
/// assert_eq!(decoded.op().mnem(), "nop");
/// assert_eq!(decoded.address(), 0x1000);
/// ```
pub fn decode(ins: u32, address: u64) -> Result<Instruction, DecodeError> {
    let (r, decoded) = unsafe {
        let mut decoded: bad64_sys::Instruction = core::mem::zeroed();
        let r = aarch64_decompose(ins, &mut decoded, address);
        (r, decoded)
    };

    match r {
        0 => {
            let op = Op::from_u32(decoded.operation as u32).unwrap();
            let mut operands: [Operand; MAX_OPERANDS as usize] =
                [Operand::Label(Imm::Unsigned(0)); MAX_OPERANDS as usize];
            let mut num_operands = 0;

            for (n, operand) in decoded.operands.iter().enumerate() {
                match Operand::try_from(operand) {
                    Ok(o) => {
                        operands[n] = o;
                        num_operands += 1;
                    }
                    Err(_) => break,
                }
            }

            Ok(Instruction {
                address,
                opcode: decoded.insword,
                op,
                num_operands,
                operands,
            })
        }
        _ => Err(DecodeError::new(r, address)),
    }
}

/// Disassemble byte slice
///
/// # Arguments
///
/// * `code` - u8 slice to zero or more instructions
/// * `address` - Location of code in memory
///
/// # Examples
/// ```
/// use bad64::{disasm, Op};
///
/// let mut decoded_iter = disasm(b"\x1f\x20\x03\xd5", 0x1000);
///
/// let decoded = decoded_iter.next().unwrap().unwrap();
///
/// assert_eq!(decoded.address(), 0x1000);
/// assert_eq!(decoded.operands().len(), 0);
/// assert_eq!(decoded.op(), Op::NOP);
/// assert_eq!(decoded.op().mnem(), "nop");
///
/// assert_eq!(decoded_iter.next(), None);
/// ```
pub fn disasm(
    code: &[u8],
    address: u64,
) -> impl Iterator<Item = Result<Instruction, DecodeError>> + '_ {
    (address..)
        .step_by(4)
        .zip(code.chunks(4))
        .map(|(addr, bytes)| match bytes.try_into() {
            Ok(v) => {
                let vv = u32::from_le_bytes(v);

                decode(vv, addr)
            }
            Err(_) => Err(DecodeError::Short(addr)),
        })
}
