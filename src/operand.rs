use core::convert::TryFrom;
use core::fmt;

use bad64_sys::*;
use cstr_core::CStr;
use num_traits::FromPrimitive;

use crate::ArrSpec;
use crate::Condition;
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

impl fmt::Display for Imm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.neg { "-" } else { "" };
        write!(f, "{}{:#x}", sign, self.val)
    }
}

impl From<&bad64_sys::InstructionOperand> for Imm {
    fn from(oo: &bad64_sys::InstructionOperand) -> Self {
        if oo.signedImm && (oo.immediate as i64) < 0 {
            Self {
                neg: true,
                val: !oo.immediate + 1,
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
    FImm32(u32),
    ShiftReg {
        reg: Reg,
        shift: Shift,
    },
    PredReg {
        reg: Reg,
        qual: char,
    },
    Reg {
        reg: Reg,
        arrspec: Option<ArrSpec>,
    },
    MultiReg {
        regs: [Option<Reg>; MAX_REGISTERS as usize],
        arrspec: Option<ArrSpec>,
    },
    SysReg(SysReg),
    MemReg(Reg),
    // TODO we might need arrspec in memoffset
    MemOffset {
        reg: Reg,
        offset: Imm,
        mul_vl: bool,
        arrspec: Option<ArrSpec>,
    },
    MemPreIdx {
        reg: Reg,
        imm: Imm,
    },
    MemPostIdxReg([Reg; 2]),
    MemPostIdxImm {
        reg: Reg,
        imm: Imm,
    },
    MemExt {
        regs: [Reg; 2],
        shift: Option<Shift>,
        arrspec: Option<ArrSpec>,
    },
    Label(Imm),
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
        imm: u64,
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
            OperandClass::FIMM32 => Ok(Self::FImm32(oo.immediate as u32)),
            OperandClass::REG => {
                let reg = Reg::from_u32(oo.reg[0] as u32).unwrap();

                if let Ok(shift) = Shift::try_from(oo) {
                    return Ok(Self::ShiftReg { reg, shift });
                }

                if oo.pred_qual != 0 {
                    return Ok(Self::PredReg { reg, qual: char::from_u32(oo.pred_qual as u32).unwrap() });
                }

                let arrspec = ArrSpec::try_from(oo).ok();

                Ok(Self::Reg { reg, arrspec })
            },
            OperandClass::MULTI_REG => {
                let mut regs = [None; MAX_REGISTERS as usize];

                for n in 0..MAX_REGISTERS as usize {
                    regs[n] = Reg::from_u32(oo.reg[n] as u32);
                }

                let arrspec = ArrSpec::try_from(oo).ok();

                Ok(Self::MultiReg {
                    regs,
                    arrspec,
                })
            }
            OperandClass::SYS_REG => Ok(Self::SysReg(SysReg::from_u32(oo.sysreg as u32).unwrap())),
            OperandClass::MEM_REG => Ok(Self::MemReg(Reg::from_u32(oo.reg[0] as u32).unwrap())),
            OperandClass::STR_IMM => Ok(Self::StrImm {
                str: oo.name.clone(),
                imm: oo.immediate,
            }),
            OperandClass::MEM_OFFSET => Ok(Self::MemOffset {
                reg: Reg::from_u32(oo.reg[0] as u32).unwrap(),
                offset: Imm::from(oo),
                mul_vl: oo.mul_vl,
                arrspec: ArrSpec::try_from(oo).ok(),
            }),
            OperandClass::MEM_PRE_IDX => {
                Ok(Self::MemPreIdx {
                    reg: Reg::from_u32(oo.reg[0] as u32).unwrap(),
                    imm: Imm::from(oo),
                })
            }
            OperandClass::MEM_POST_IDX => {
                let reg0 = Reg::from_u32(oo.reg[0] as u32).unwrap();

                match Reg::from_u32(oo.reg[1] as u32) {
                    Some(reg1) => Ok(Self::MemPostIdxReg([reg0, reg1])),
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
                arrspec: ArrSpec::try_from(oo).ok(),
            }),
            OperandClass::LABEL => Ok(Self::Label(Imm::from(oo))),
            OperandClass::IMPLEMENTATION_SPECIFIC => Ok(Self::ImplSpec {
                o0: oo.implspec[0],
                o1: oo.implspec[1],
                cm: oo.implspec[2],
                cn: oo.implspec[3],
                o2: oo.implspec[4],
            }),
            OperandClass::CONDITION => Ok(Self::Cond(Condition::from_u32(oo.cond as u32).unwrap())),
            OperandClass::NAME => Ok(Self::Name(oo.name.clone())),
            OperandClass::NONE => Err(()),
        }
    }
}

fn write_full_reg(f: &mut fmt::Formatter<'_>, reg: Reg, arrspec: Option<ArrSpec>) -> fmt::Result {
    match arrspec {
        Some(arsp) => write!(f, "{}{}", reg, arsp.suffix(reg)),
        None => write!(f, "{}", reg),
    }
}

fn write_full_reg_lane(f: &mut fmt::Formatter<'_>, reg: Reg, arrspec: Option<ArrSpec>) -> fmt::Result {
    match arrspec {
        Some(arsp) => {
            match arsp.lane() {
                Some(lane) => write!(f, "{}{}[{}]", reg, arsp.suffix(reg), lane),
                None => write!(f, "{}{}", reg, arsp.suffix(reg)),
            }
        }
        None => write!(f, "{}", reg),
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Imm64 { imm, shift } | Self::Imm32 { imm, shift } => {
                write!(f, "#{}", imm)?;

                if let Some(s) = shift {
                    write!(f, ", {}", s)?;
                }

                Ok(())
            }
            Self::FImm32(ff) => write!(f, "#{}", f32::from_le_bytes(ff.to_le_bytes())),
            Self::ShiftReg { reg, shift } => write!(f, "{}, {}", reg, shift),
            Self::PredReg { reg, qual } => write!(f, "{}/{}", reg, qual),
            Self::Reg { reg, arrspec } => write_full_reg_lane(f, reg, arrspec),
            Self::MultiReg { regs, arrspec } => {
                write!(f, "{{")?;

                let mut num_regs = 0;

                // count the reigsters...
                for n in 0..MAX_REGISTERS as usize {
                    if regs[n].is_some() {
                        num_regs += 1;
                    } else {
                        break;
                    }
                }

                for n in 0..num_regs {
                    let reg = regs[n].unwrap();

                    if n != num_regs - 1 {
                        write_full_reg(f, reg, arrspec)?;
                        write!(f, ", ")?;
                    } else {
                        // last
                        write_full_reg(f, reg, arrspec)?;
                        write!(f, "}}")?;
                    }
                }

                if let Some(arsp) = arrspec {
                    if let Some(lane) = arsp.lane() {
                        write!(f, "[{}]", lane)?;
                    }
                }

                Ok(())
            }
            Self::SysReg(sr) => write!(f, "{:.08}", sr),
            Self::MemReg(mr) => write!(f, "[{}]", mr),
            Self::MemPreIdx { reg, imm } => write!(f, "[{}, #{}]!", reg, imm),
            Self::MemPostIdxImm { reg, imm } => write!(f, "[{}], #{}", reg, imm),
            Self::MemExt { regs, shift, arrspec } => {
                write!(f, "[")?;
                write_full_reg(f, regs[0], arrspec )?;
                write!(f, ", ")?;
                write_full_reg(f, regs[1], arrspec )?;

                match shift {
                    Some(ss) => write!(f, ", {}]", ss),
                    None => write!(f, "]"),
                }
            }
            Self::MemPostIdxReg(regs) => write!(f, "[{}], {}", regs[0], regs[1]),
            // TODO see if I need arrspec
            Self::MemOffset { reg, offset, arrspec, mul_vl } => {
                write!(f, "[")?;
                write_full_reg(f, reg, arrspec)?;

                if offset.val != 0 {
                    write!(f, ", #{}", offset)?;

                    if mul_vl {
                        write!(f, ", mul vl")?;
                    }
                }
                write!(f, "]")
            },
            Self::Label(imm) => write!(f, "{}", imm),
            Self::ImplSpec { o0, o1, cm, cn, o2 } => write!(f, "s{}_{}_c{}_c{}_{}", o0, o1, cm, cn, o2),
            Self::Cond(c) => write!(f, "{}", c),
            Self::Name(str) => {
                let name = unsafe { CStr::from_ptr(str.as_ptr()) }.to_str().unwrap();

                write!(f, "{}", name)
            }
            Self::StrImm { str, imm } => {
                let name = unsafe { CStr::from_ptr(str.as_ptr()) }.to_str().unwrap();

                write!(f, "{} #{:#x}", name, imm)
            }
        }
    }
}
