use core::convert::TryFrom;

use bad64_sys::*;

/// A shift applied to a register or immediate
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

#[doc(hidden)]
#[allow(non_upper_case_globals)]
impl TryFrom<&bad64_sys::InstructionOperand> for Shift {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        match oo.shiftType {
            ShiftType::ShiftType_LSL => Ok(Shift::LSL(oo.shiftValue)),
            ShiftType::ShiftType_LSR => Ok(Shift::LSR(oo.shiftValue)),
            ShiftType::ShiftType_ASR => Ok(Shift::ASR(oo.shiftValue)),
            ShiftType::ShiftType_ROR => Ok(Shift::ROR(oo.shiftValue)),
            ShiftType::ShiftType_UXTW => Ok(Shift::UXTW(oo.shiftValue)),
            ShiftType::ShiftType_SXTW => Ok(Shift::SXTW(oo.shiftValue)),
            ShiftType::ShiftType_UXTX => Ok(Shift::UXTX(oo.shiftValue)),
            ShiftType::ShiftType_SXTX => Ok(Shift::SXTX(oo.shiftValue)),
            ShiftType::ShiftType_SXTB => Ok(Shift::SXTB(oo.shiftValue)),
            ShiftType::ShiftType_SXTH => Ok(Shift::SXTH(oo.shiftValue)),
            ShiftType::ShiftType_UXTH => Ok(Shift::UXTH(oo.shiftValue)),
            ShiftType::ShiftType_UXTB => Ok(Shift::UXTB(oo.shiftValue)),
            ShiftType::ShiftType_MSL => Ok(Shift::MSL(oo.shiftValue)),
            ShiftType::ShiftType_NONE | ShiftType::ShiftType_END => Err(()),
        }
    }
}
