use core::convert::TryFrom;

use bad64_sys::*;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Condition {
    EQ = Condition_COND_EQ,
    NE = Condition_COND_NE,
    CS = Condition_COND_CS,
    CC = Condition_COND_CC,
    MI = Condition_COND_MI,
    PL = Condition_COND_PL,
    VS = Condition_COND_VS,
    VC = Condition_COND_VC,
    HI = Condition_COND_HI,
    LS = Condition_COND_LS,
    GE = Condition_COND_GE,
    LT = Condition_COND_LT,
    GT = Condition_COND_GT,
    LE = Condition_COND_LE,
    AL = Condition_COND_AL,
    NV = Condition_COND_NV
}