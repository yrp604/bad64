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
    None = OperandClass_NONE,
    Imm32 = OperandClass_IMM32,
    Imm64 = OperandClass_IMM64,
    FImm32 = OperandClass_FIMM32,
    StrImm = OperandClass_STR_IMM,
    Reg = OperandClass_REG,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SysReg(i32);

impl SysReg {
    pub fn name(&self) -> &'static str {
        unsafe { CStr::from_ptr(get_system_register_name(self.0)) }
            .to_str()
            .unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Reg(i32);

impl Reg {
    pub fn name(&self) -> &'static str {
        unsafe { CStr::from_ptr(get_register_name(self.0)) }
            .to_str()
            .unwrap()
    }

    pub fn size(&self) -> usize {
        unsafe { get_register_size(self.0) as usize }
    }
}

/// Structure containing an instruction operand
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Operand<'a>(&'a bad64_sys::InstructionOperand);

impl Operand<'_> {
    pub fn class(&self) -> OperandClass {
        OperandClass::from_i32(self.0.operandClass).expect("unknown operand class in operand")
    }

    pub fn reg(&self, n: usize) -> Option<Reg> {
        match self.class() {
            OperandClass::Reg | OperandClass::MemReg | OperandClass::MultiReg => {
                // TODO: add MAX_REGISTERS when it gets implemented
                if n >= 5 {
                    return None;
                }

                if self.0.reg[n] == Register_REG_NONE {
                    return None;
                }

                Some(Reg(self.0.reg[n]))
            }
            _ => None,
        }
    }

    pub fn imm(&self) -> Option<u64> {
        match self.class() {
            OperandClass::Imm32
            | OperandClass::Imm64
            | OperandClass::FImm32
            | OperandClass::StrImm => Some(self.0.immediate),
            _ => None,
        }
    }

    pub fn sysreg(&self) -> Option<SysReg> {
        if self.class() != OperandClass::SysReg {
            return None;
        }

        Some(SysReg(self.0.sysreg))
    }
}

/// Structure containing a decoded instruction
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction(bad64_sys::Instruction);

impl Instruction {
    /// Get the instruction mnemonic
    pub fn mnem(&self) -> &'static str {
        unsafe { CStr::from_ptr(get_operation(&self.0 as _)) }
            .to_str()
            .unwrap()
    }

    pub fn operand(&self, n: usize) -> Option<Operand> {
        if n >= MAX_OPERANDS as usize {
            return None;
        }

        let o = Operand(&self.0.operands[n]);

        if o.class() == OperandClass::None {
            return None;
        }

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
