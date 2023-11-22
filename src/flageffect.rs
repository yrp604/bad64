/// The semantic meaning of the flag setting
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlagEffect {
    Sets,
    Integer,
    Float,
}

#[doc(hidden)]
#[allow(non_upper_case_globals)]
impl TryFrom<&bad64_sys::Instruction> for FlagEffect {
    type Error = ();

    fn try_from(ii: &bad64_sys::Instruction) -> Result<Self, Self::Error> {
        match ii.setflags {
            bad64_sys::FlagEffect::FLAGEFFECT_SETS => Ok(FlagEffect::Sets),
            bad64_sys::FlagEffect::FLAGEFFECT_SETS_NORMAL => Ok(FlagEffect::Integer),
            bad64_sys::FlagEffect::FLAGEFFECT_SETS_FLOAT => Ok(FlagEffect::Float),
            bad64_sys::FlagEffect::FLAGEFFECT_NONE => Err(()),
        }
    }
}
