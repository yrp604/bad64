#![no_std]

#[macro_use]
extern crate num_derive;

use core::mem::MaybeUninit;

use cstr_core::CStr;
use num_traits::FromPrimitive;

use bad64_sys::*;
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum OperandClass {
    None =  OperandClass_NONE,
    Imm32 = OperandClass_IMM32,
    Imm64 = OperandClass_IMM64,
    FImm32 = OperandClass_FIMM32,
    StrImm = OperandClass_STR_IMM,
    MultiReg = OperandClass_MULTI_REG,
    SysReg = OperandClass_SYS_REG,
    MemReg = OperandClass_MEM_REG,
    MemPreIdx = OperandClass_MEM_PRE_IDX,
    MemPostIdx = OperandClass_MEM_POST_IDX,
    MemOffset = OperandClass_MEM_OFFSET,
    MemExtended = OperandClass_MEM_EXTENDED,
    Label = OperandClass_LABEL,
    Condition = OperandClass_CONDITION,
    Name = OperandClass_NAME,
    ImplementationSpecific = OperandClass_IMPLEMENTATION_SPECIFIC,
}

/// Structure containing an instruction operand
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Operand<'a>(&'a bad64_sys::InstructionOperand);

impl Operand<'_> {
    pub fn class(&self) -> OperandClass {
        OperandClass::from_i32(self.0.operandClass).unwrap()
    }
}

/// Structure containing a decoded instruction
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction(bad64_sys::Instruction);

impl Instruction {
    /// Get the instruction mnemonic
    pub fn mnem(&self) -> &str {
        unsafe { CStr::from_ptr(get_operation(&self.0 as _)) }
            .to_str()
            .unwrap()
    }

    pub fn operand(&self, n: usize) -> Option<Operand> {
        if n >= 5 { return None; }

        let o = Operand(&self.0.operands[n]);

        if o.class() == OperandClass::None { return None; }

        Some(o)
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
    let mut decoded = MaybeUninit::uninit();

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