use core::convert::TryFrom;

use bad64_sys::*;

use crate::Reg;

/// An arrangement specifier
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArrSpec {
    Full(Option<u32>),
    TwoDoubles(Option<u32>),
    FourSingles(Option<u32>),
    EightHalves(Option<u32>),
    SixteenBytes(Option<u32>),
    OneDouble(Option<u32>),
    TwoSingles(Option<u32>),
    FourHalves(Option<u32>),
    EightBytes(Option<u32>),
    OneSingle(Option<u32>),
    TwoHalves(Option<u32>),
    FourBytes(Option<u32>),
    OneHalf(Option<u32>),
    OneByte(Option<u32>),
}

#[allow(non_upper_case_globals)]
#[doc(hidden)]
impl TryFrom<&bad64_sys::InstructionOperand> for ArrSpec {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        let lane = match oo.laneUsed {
            true => Some(oo.lane),
            false => None,
        };

        match oo.arrSpec {
            ArrangementSpec::ARRSPEC_FULL => Ok(ArrSpec::Full(lane)),
            ArrangementSpec::ARRSPEC_2DOUBLES => Ok(ArrSpec::TwoDoubles(lane)),
            ArrangementSpec::ARRSPEC_4SINGLES => Ok(ArrSpec::FourSingles(lane)),
            ArrangementSpec::ARRSPEC_8HALVES => Ok(ArrSpec::EightHalves(lane)),
            ArrangementSpec::ARRSPEC_16BYTES => Ok(ArrSpec::SixteenBytes(lane)),
            ArrangementSpec::ARRSPEC_1DOUBLE => Ok(ArrSpec::OneDouble(lane)),
            ArrangementSpec::ARRSPEC_2SINGLES => Ok(ArrSpec::TwoSingles(lane)),
            ArrangementSpec::ARRSPEC_4HALVES => Ok(ArrSpec::FourHalves(lane)),
            ArrangementSpec::ARRSPEC_8BYTES => Ok(ArrSpec::EightBytes(lane)),
            ArrangementSpec::ARRSPEC_1SINGLE => Ok(ArrSpec::OneSingle(lane)),
            ArrangementSpec::ARRSPEC_2HALVES => Ok(ArrSpec::TwoHalves(lane)),
            ArrangementSpec::ARRSPEC_4BYTES => Ok(ArrSpec::FourBytes(lane)),
            ArrangementSpec::ARRSPEC_1HALF => Ok(ArrSpec::OneHalf(lane)),
            ArrangementSpec::ARRSPEC_1BYTE => Ok(ArrSpec::OneByte(lane)),
            ArrangementSpec::ARRSPEC_NONE => Err(()),
        }
    }
}

impl ArrSpec {
    pub fn lane(&self) -> Option<u32> {
        return match *self {
            Self::Full(lane)
            | Self::TwoDoubles(lane)
            | Self::OneDouble(lane)
            | Self::FourSingles(lane)
            | Self::TwoSingles(lane)
            | Self::OneSingle(lane)
            | Self::EightHalves(lane)
            | Self::FourHalves(lane)
            | Self::TwoHalves(lane)
            | Self::OneHalf(lane)
            | Self::SixteenBytes(lane)
            | Self::EightBytes(lane)
            | Self::FourBytes(lane)
            | Self::OneByte(lane) => lane,
        };
    }

    pub fn suffix(&self, reg: Reg) -> &'static str {
        let is_sve = reg.is_sve();
        let is_pred = reg.is_pred();

        if !reg.is_simd() && !is_sve && !is_pred {
            return "";
        }

        if self.lane().is_some() || is_sve || is_pred {
            return match *self {
                Self::Full(_) => ".q",
                Self::TwoDoubles(_) | Self::OneDouble(_) => ".d",
                Self::FourSingles(_) | Self::TwoSingles(_) | Self::OneSingle(_) => ".s",
                Self::EightHalves(_)
                | Self::FourHalves(_)
                | Self::TwoHalves(_)
                | Self::OneHalf(_) => ".h",
                Self::SixteenBytes(_)
                | Self::EightBytes(_)
                | Self::FourBytes(_)
                | Self::OneByte(_) => ".b",
            };
        }

        return match *self {
            Self::Full(_) => ".1q",
            Self::TwoDoubles(_) => ".2d",
            Self::OneDouble(_) => ".1d",
            Self::FourSingles(_) => ".4s",
            Self::TwoSingles(_) => ".2s",
            Self::OneSingle(_) => ".1s",
            Self::EightHalves(_) => ".8h",
            Self::FourHalves(_) => ".4h",
            Self::TwoHalves(_) => ".2h",
            Self::OneHalf(_) => ".1h",
            Self::SixteenBytes(_) => ".16b",
            Self::EightBytes(_) => ".8b",
            Self::FourBytes(_) => ".4b",
            Self::OneByte(_) => ".1b",
        };
    }
}
