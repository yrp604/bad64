use core::convert::TryFrom;

use bad64_sys::*;
use num_traits::FromPrimitive;

use crate::Reg;
use crate::SysReg;

/// Structure containing an instruction operand
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operand {
    Imm32(u32),
    Imm64(u64),
    FImm32(u64),
    StrImm(u64),
    Reg(Reg),
    MultiReg([Option<Reg>; 5]), // TODO : MAX_REGISTERS
    SysReg(SysReg),
    ImplementationSpecific(u8, u8, u8, u8, u8),
    /*
    StrImm = OperandClass_STR_IMM,
    MemReg = OperandClass_MEM_REG,
    MemPreIdx = OperandClass_MEM_PRE_IDX,
    MemPostIdx = OperandClass_MEM_POST_IDX,
    MemOffset = OperandClass_MEM_OFFSET,
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
            OperandClass_IMM32 => Ok(Self::Imm32(oo.immediate as u32)),
            OperandClass_IMM64 => Ok(Self::Imm64(oo.immediate)),
            OperandClass_FIMM32 => Ok(Self::FImm32(oo.immediate)),
            OperandClass_REG => Ok(Self::Reg(Reg::from_i32(oo.reg[0]).unwrap())),
            OperandClass_MULTI_REG => {
                let mut regs = [None; 5];

                for n in 0..5 {
                    // TODO: MAX_REGS
                    if oo.reg[n] != Register_REG_NONE {
                        regs[n] = Some(Reg::from_i32(oo.reg[n]).unwrap());
                    }
                }

                Ok(Self::MultiReg(regs))
            }
            OperandClass_SYS_REG => Ok(Self::SysReg(SysReg::from_i32(oo.sysreg).unwrap())),
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
