use core::convert::TryFrom;

use bad64_sys::*;
use num_traits::FromPrimitive;

use crate::Reg;
use crate::Shift;
use crate::SysReg;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Imm {
    pub neg: bool,
    pub val: u64,
}

impl From<&bad64_sys::InstructionOperand> for Imm {
    fn from(oo: &bad64_sys::InstructionOperand) -> Self {
        if oo.signedImm == 1 && (oo.immediate as i64) < 0 {
            Self {
                neg: true,
                val: !oo.immediate,
            }
        } else {
            Self {
                neg: false,
                val: oo.immediate,
            }
        }
    }
}

/// Structure containing an instruction operand
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operand {
    Imm32(Imm, Option<Shift>),
    Imm64(Imm, Option<Shift>),
    FImm32(Imm, Option<Shift>),
    StrImm(Imm, Option<Shift>),
    Reg(Reg, Option<Shift>),
    MultiReg([Option<Reg>; 5], Option<u32>), // TODO : MAX_REGISTERS
    SysReg(SysReg),
    MemReg(Reg),
    MemOffset(Reg, u64),
    MemPreIdx(Reg, i64),
    Label(Imm, Option<Shift>),
    ImplementationSpecific(u8, u8, u8, u8, u8),
    /*
    MemPostIdx = OperandClass_MEM_POST_IDX,
    MemExtended = OperandClass_MEM_EXTENDED,
    Condition = OperandClass_CONDITION,
    Name = OperandClass_NAME,
    */
}

#[allow(non_upper_case_globals)]
impl TryFrom<&bad64_sys::InstructionOperand> for Operand {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        match oo.operandClass {
            OperandClass_IMM32 => Ok(Self::Imm32(Imm::from(oo), Shift::try_from(oo).ok())),
            OperandClass_IMM64 => Ok(Self::Imm64(Imm::from(oo), Shift::try_from(oo).ok())),
            OperandClass_FIMM32 => Ok(Self::FImm32(Imm::from(oo), Shift::try_from(oo).ok())),
            OperandClass_REG => Ok(Self::Reg(
                Reg::from_i32(oo.reg[0]).unwrap(),
                Shift::try_from(oo).ok(),
            )),
            OperandClass_MULTI_REG => {
                let mut regs = [None; 5];

                for n in 0..5 {
                    // TODO: MAX_REGS
                    regs[n] = Reg::from_i32(oo.reg[n]);
                }

                let lane = match oo.laneUsed {
                    true => Some(oo.lane),
                    false => None,
                };

                Ok(Self::MultiReg(regs, lane))
            }
            OperandClass_SYS_REG => Ok(Self::SysReg(SysReg::from_i32(oo.sysreg).unwrap())),
            OperandClass_MEM_REG => Ok(Self::MemReg(Reg::from_i32(oo.reg[0]).unwrap())),
            // XXX WRONG shoudl be name + immediate?
            //OperandClass_STR_IMM => Ok(Self::StrImm(oo.immediate)),
            OperandClass_MEM_OFFSET => Ok(Self::MemOffset(
                Reg::from_i32(oo.reg[0]).unwrap(),
                oo.immediate,
            )),
            OperandClass_MEM_PRE_IDX => {
                let off = match oo.signedImm {
                    0 => oo.immediate as i64,
                    _ => -(oo.immediate as i64),
                };

                Ok(Self::MemPreIdx(Reg::from_i32(oo.reg[0]).unwrap(), off))
            }
            OperandClass_LABEL => Ok(Self::Label(Imm::from(oo), Shift::try_from(oo).ok())),
            OperandClass_IMPLEMENTATION_SPECIFIC => Ok(Self::ImplementationSpecific(
                oo.implspec[0],
                oo.implspec[1],
                oo.implspec[2],
                oo.implspec[3],
                oo.implspec[4],
            )),
            _ => Err(()),
        }
    }
}
