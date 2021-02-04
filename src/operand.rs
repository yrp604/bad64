use core::convert::TryFrom;

use bad64_sys::*;
use num_traits::FromPrimitive;

use crate::Reg;
use crate::Shift;
use crate::SysReg;

/// An instruction immediate
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Imm {
    /// Did the disassembler consider this value negative
    pub neg: bool,
    /// The immediate value
    pub val: u64,
}

impl From<&bad64_sys::InstructionOperand> for Imm {
    fn from(oo: &bad64_sys::InstructionOperand) -> Self {
        if oo.signedImm && (oo.immediate as i64) < 0 {
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

/// An instruction operand
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operand {
    Imm32 {
        imm: Imm,
        shift: Option<Shift>,
    },
    Imm64 {
        imm: Imm,
        shift: Option<Shift>,
    },
    FImm32 {
        imm: Imm,
        shift: Option<Shift>,
    },
    Reg {
        reg: Reg,
        shift: Option<Shift>,
    },
    MultiReg {
        regs: [Option<Reg>; MAX_REGISTERS as usize],
        lane: Option<u32>,
    },
    SysReg(SysReg),
    MemReg(Reg),
    MemOffset {
        reg: Reg,
        offset: u64,
        mul_vl: bool,
    },
    MemPreIdx {
        reg: Reg,
        offset: i64,
    },
    MemPostIdxReg {
        regs: [Reg; 2],
    },
    MemPostIdxImm {
        reg: Reg,
        imm: Imm,
    },
    MemExt {
        regs: [Reg; 2],
        shift: Option<Shift>,
    },
    Label {
        imm: Imm,
        shift: Option<Shift>,
    },
    ImplSpec {
        o0: u8,
        o1: u8,
        cm: u8,
        cn: u8,
        o2: u8,
    },
    Cond(Condition),
    Name([i8; MAX_NAME as usize]),
    StrImm {
        str: [i8; MAX_NAME as usize],
        imm: Imm,
    },
}

#[allow(non_upper_case_globals)]
#[doc(hidden)]
impl TryFrom<&bad64_sys::InstructionOperand> for Operand {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        match oo.operandClass {
            OperandClass::IMM32 => Ok(Self::Imm32 {
                imm: Imm::from(oo),
                shift: Shift::try_from(oo).ok(),
            }),
            OperandClass::IMM64 => Ok(Self::Imm64 {
                imm: Imm::from(oo),
                shift: Shift::try_from(oo).ok(),
            }),
            OperandClass::FIMM32 => Ok(Self::FImm32 {
                imm: Imm::from(oo),
                shift: Shift::try_from(oo).ok(),
            }),
            OperandClass::REG => Ok(Self::Reg {
                reg: Reg::from_u32(oo.reg[0] as u32).unwrap(),
                shift: Shift::try_from(oo).ok(),
            }),
            OperandClass::MULTI_REG => {
                let mut regs = [None; MAX_REGISTERS as usize];

                for n in 0..MAX_REGISTERS as usize {
                    regs[n] = Reg::from_u32(oo.reg[n] as u32);
                }

                let lane = match oo.laneUsed {
                    true => Some(oo.lane),
                    false => None,
                };

                Ok(Self::MultiReg { regs, lane })
            }
            OperandClass::SYS_REG => Ok(Self::SysReg(SysReg::from_u32(oo.sysreg as u32).unwrap())),
            OperandClass::MEM_REG => Ok(Self::MemReg(Reg::from_u32(oo.reg[0] as u32).unwrap())),
            OperandClass::STR_IMM => Ok(Self::StrImm {
                str: oo.name.clone(),
                imm: Imm::from(oo),
            }),
            OperandClass::MEM_OFFSET => Ok(Self::MemOffset {
                reg: Reg::from_u32(oo.reg[0] as u32).unwrap(),
                offset: oo.immediate,
                mul_vl: oo.mul_vl,
            }),
            OperandClass::MEM_PRE_IDX => {
                let off = if oo.signedImm {
                    -(oo.immediate as i64)
                } else {
                    oo.immediate as i64
                };

                Ok(Self::MemPreIdx {
                    reg: Reg::from_u32(oo.reg[0] as u32).unwrap(),
                    offset: off,
                })
            }
            OperandClass::MEM_POST_IDX => {
                let reg0 = Reg::from_u32(oo.reg[0] as u32).unwrap();

                match Reg::from_u32(oo.reg[1] as u32) {
                    Some(reg1) => Ok(Self::MemPostIdxReg { regs: [reg0, reg1] }),
                    None => Ok(Self::MemPostIdxImm {
                        reg: reg0,
                        imm: Imm::from(oo),
                    }),
                }
            }
            OperandClass::MEM_EXTENDED => Ok(Self::MemExt {
                regs: [
                    Reg::from_u32(oo.reg[0] as u32).unwrap(),
                    Reg::from_u32(oo.reg[1] as u32).unwrap(),
                ],
                shift: Shift::try_from(oo).ok(),
            }),
            OperandClass::LABEL => Ok(Self::Label {
                imm: Imm::from(oo),
                shift: Shift::try_from(oo).ok(),
            }),
            OperandClass::IMPLEMENTATION_SPECIFIC => Ok(Self::ImplSpec {
                o0: oo.implspec[0],
                o1: oo.implspec[1],
                cm: oo.implspec[2],
                cn: oo.implspec[3],
                o2: oo.implspec[4],
            }),
            OperandClass::CONDITION => Ok(Self::Cond(
                Condition::from_u32(oo.cond as u32).unwrap(),
            )),
            OperandClass::NAME => Ok(Self::Name(oo.name.clone())),
            OperandClass::NONE => Err(()),
        }
    }
}
