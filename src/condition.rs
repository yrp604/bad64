use bad64_sys::*;

use core::fmt;

/// A condition
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum Condition {
    EQ = Condition_COND_EQ as u32,
    NE = Condition_COND_NE as u32,
    CS = Condition_COND_CS as u32,
    CC = Condition_COND_CC as u32,
    MI = Condition_COND_MI as u32,
    PL = Condition_COND_PL as u32,
    VS = Condition_COND_VS as u32,
    VC = Condition_COND_VC as u32,
    HI = Condition_COND_HI as u32,
    LS = Condition_COND_LS as u32,
    GE = Condition_COND_GE as u32,
    LT = Condition_COND_LT as u32,
    GT = Condition_COND_GT as u32,
    LE = Condition_COND_LE as u32,
    AL = Condition_COND_AL as u32,
    NV = Condition_COND_NV as u32,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EQ => write!(f, "eq"),
            Self::NE => write!(f, "ne"),
            Self::CS => write!(f, "cs"),
            Self::CC => write!(f, "cc"),
            Self::MI => write!(f, "mi"),
            Self::PL => write!(f, "pl"),
            Self::VS => write!(f, "vs"),
            Self::VC => write!(f, "vc"),
            Self::HI => write!(f, "hi"),
            Self::LS => write!(f, "ls"),
            Self::GE => write!(f, "ge"),
            Self::LT => write!(f, "lt"),
            Self::GT => write!(f, "gt"),
            Self::LE => write!(f, "le"),
            Self::AL => write!(f, "al"),
            Self::NV => write!(f, "nv"),
        }
    }
}
