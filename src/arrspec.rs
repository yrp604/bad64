use core::convert::TryFrom;

use num_traits::ToPrimitive;

use bad64_sys::*;

use crate::Reg;

/// An arrangement specifier
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArrSpec {
    Full,
    TwoDoubles,
    FourSingles,
    EightHalves,
    SixteenBytes,
    OneDouble,
    TwoSingles,
    FourHalves,
    EightBytes,
    OneSingle,
    TwoHalves,
    FourBytes,
    OneHalf,
    OneByte,
}

#[allow(non_upper_case_globals)]
#[doc(hidden)]
impl TryFrom<&bad64_sys::InstructionOperand> for ArrSpec {
    type Error = ();

    fn try_from(oo: &bad64_sys::InstructionOperand) -> Result<Self, Self::Error> {
        match oo.arrSpec {
            ArrangementSpec::ARRSPEC_FULL => Ok(ArrSpec::Full),
            ArrangementSpec::ARRSPEC_2DOUBLES => Ok(ArrSpec::TwoDoubles),
            ArrangementSpec::ARRSPEC_4SINGLES => Ok(ArrSpec::FourSingles),
            ArrangementSpec::ARRSPEC_8HALVES => Ok(ArrSpec::EightHalves),
            ArrangementSpec::ARRSPEC_16BYTES => Ok(ArrSpec::SixteenBytes),
            ArrangementSpec::ARRSPEC_1DOUBLE => Ok(ArrSpec::OneDouble),
            ArrangementSpec::ARRSPEC_2SINGLES => Ok(ArrSpec::TwoSingles),
            ArrangementSpec::ARRSPEC_4HALVES => Ok(ArrSpec::FourHalves),
            ArrangementSpec::ARRSPEC_8BYTES => Ok(ArrSpec::EightBytes),
            ArrangementSpec::ARRSPEC_1SINGLE => Ok(ArrSpec::OneSingle),
            ArrangementSpec::ARRSPEC_2HALVES => Ok(ArrSpec::TwoHalves),
            ArrangementSpec::ARRSPEC_4BYTES => Ok(ArrSpec::FourBytes),
            ArrangementSpec::ARRSPEC_1HALF => Ok(ArrSpec::OneHalf),
            ArrangementSpec::ARRSPEC_1BYTE => Ok(ArrSpec::OneByte),
            ArrangementSpec::ARRSPEC_NONE => Err(()),
        }
    }
}

impl ArrSpec {
    pub fn reg_suffix(&self, reg: Reg, lane: Option<u32>) -> &'static str {
        let regno = reg.to_u32().unwrap();

        let is_simd = regno >= Reg::V0.to_u32().unwrap() && regno <= Reg::V31.to_u32().unwrap();
        let is_sve = regno >= Reg::Z0.to_u32().unwrap() && regno <= Reg::Z31.to_u32().unwrap();
        let is_pred = regno >= Reg::P0.to_u32().unwrap() && regno <= Reg::P31.to_u32().unwrap();

        if !is_simd && !is_sve && !is_pred {
            return "";
        }

        if lane.is_some() || is_sve || is_pred {
            return match *self {
                Self::Full => ".q",
                Self::TwoDoubles | Self::OneDouble => ".d",
                Self::FourSingles | Self::TwoSingles | Self::OneSingle => ".s",
                Self::EightHalves | Self::FourHalves | Self::TwoHalves | Self::OneHalf => ".h",
                Self::SixteenBytes | Self::EightBytes | Self::FourBytes | Self::OneByte => ".b",
            };
        }

        return match *self {
            Self::Full => ".1q",
            Self::TwoDoubles => ".2d",
            Self::OneDouble => ".1d",
            Self::FourSingles => ".4s",
            Self::TwoSingles => ".2s",
            Self::OneSingle => ".1s",
            Self::EightHalves => ".8h",
            Self::FourHalves => ".4h",
            Self::TwoHalves => ".2h",
            Self::OneHalf => ".1h",
            Self::SixteenBytes => ".16b",
            Self::EightBytes => ".8b",
            Self::FourBytes => ".4b",
            Self::OneByte => ".1b",
        };
    }
}
