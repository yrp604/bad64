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

impl<'a> Operand<'a> {
    pub fn new(r: &'a bad64_sys::InstructionOperand) -> Self {
        Self(r)
    }
    pub fn class(&self) -> OperandClass {
        OperandClass::from_i32(self.0.operandClass).expect("unknown operand class in operand")
    }

    pub fn reg(&self, n: usize) -> Option<Reg> {
        match self.class() {
            OperandClass::Reg | OperandClass::MultiReg => {
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
    /*
    pub fn mem(&self) -> Option<()> {
        match self.class() {
            OperandClass::MemReg
            | OperandClass::MemOffset
            | OperandClass::MemExtended
            | OperandClass::MemPreIdx
            | OperandClass::MemPostIdx => {

            }
            _ => None,
        }
    }
    */

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

    pub fn implspec(&self) -> Option<(u8, u8, u8, u8, u8)> {
        if self.class() != OperandClass::ImplementationSpecific {
            return None;
        }

        Some((
            self.0.implspec[0],
            self.0.implspec[1],
            self.0.implspec[2],
            self.0.implspec[3],
            self.0.implspec[4],
        ))
    }
}
