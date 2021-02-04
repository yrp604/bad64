use core::convert::TryFrom;

use bad64_sys::*;

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
