use core::convert::TryFrom;

use bad64_sys::*;
use num_traits::FromPrimitive;

use crate::Reg;
use crate::SysReg;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Shift {
    LSL(u32),
    LSR(u32),
    ASR(u32),
    ROR(u32),
    UXTW(u32),
    SXTW(u32),
    SXTX(u32),
    UXTX(u32),
    SXTB(u32),
    SXTH(u32),
    UXTH(u32),
    UXTB(u32),
    MSL(u32),
}

#[allow(non_upper_case_globals)]
impl TryFrom<&bad64_sys::InstructionOperand> for Shift {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        match oo.shiftType {
            ShiftType_ShiftType_LSL => Ok(Shift::LSL(oo.shiftValue)),
            ShiftType_ShiftType_LSR => Ok(Shift::LSR(oo.shiftValue)),
            ShiftType_ShiftType_ASR => Ok(Shift::ASR(oo.shiftValue)),
            ShiftType_ShiftType_ROR => Ok(Shift::ROR(oo.shiftValue)),
            ShiftType_ShiftType_UXTW => Ok(Shift::UXTW(oo.shiftValue)),
            ShiftType_ShiftType_SXTW => Ok(Shift::SXTW(oo.shiftValue)),
            ShiftType_ShiftType_UXTX => Ok(Shift::UXTX(oo.shiftValue)),
            ShiftType_ShiftType_SXTX => Ok(Shift::SXTX(oo.shiftValue)),
            ShiftType_ShiftType_SXTB => Ok(Shift::SXTB(oo.shiftValue)),
            ShiftType_ShiftType_SXTH => Ok(Shift::SXTH(oo.shiftValue)),
            ShiftType_ShiftType_UXTH => Ok(Shift::UXTH(oo.shiftValue)),
            ShiftType_ShiftType_UXTB => Ok(Shift::UXTB(oo.shiftValue)),
            ShiftType_ShiftType_MSL => Ok(Shift::MSL(oo.shiftValue)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Imm{
    pub neg: bool,
    pub val: u64,
    pub shift: Option<Shift>
}

impl From<&bad64_sys::InstructionOperand> for Imm {
    fn from(oo: &bad64_sys::InstructionOperand) -> Self {
        if oo.signedImm == 1 && (oo.immediate as i64) < 0 {
            Self { neg: true, val: !oo.immediate, shift: Shift::try_from(oo).ok() }
        } else {
            Self { neg: false, val: oo.immediate, shift: Shift::try_from(oo).ok() }
        }
    }
}

/// Structure containing an instruction operand
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operand {
    Imm32(Imm),
    Imm64(Imm),
    FImm32(Imm),
    StrImm(Imm),
    Reg(Reg),
    MultiReg([Option<Reg>; 5], Option<u32>), // TODO : MAX_REGISTERS
    SysReg(SysReg),
    MemReg(Reg),
    MemOffset(Reg, u64),
    MemPreIdx(Reg, i64),
    Label(Imm),
    ImplementationSpecific(u8, u8, u8, u8, u8),
    /*
    MemPostIdx = OperandClass_MEM_POST_IDX,
    MemExtended = OperandClass_MEM_EXTENDED,
    Label = OperandClass_LABEL,
    Condition = OperandClass_CONDITION,
    Name = OperandClass_NAME,
    */
}

#[allow(non_upper_case_globals)]
impl TryFrom<&bad64_sys::InstructionOperand> for Operand {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        match oo.operandClass {
            OperandClass_IMM32 => Ok(Self::Imm32(Imm::from(oo))),
            OperandClass_IMM64 => Ok(Self::Imm64(Imm::from(oo))),
            OperandClass_FIMM32 => Ok(Self::FImm32(Imm::from(oo))),
            OperandClass_REG => Ok(Self::Reg(Reg::from_i32(oo.reg[0]).unwrap())),
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
            OperandClass_LABEL => Ok(Self::Label(Imm::from(oo))),
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
