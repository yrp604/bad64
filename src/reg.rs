use core::fmt;

use cstr_core::CStr;
use num_traits::ToPrimitive;

use bad64_sys::*;

/// A register
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum Reg {
    W0 = Register_REG_W0 as u32,
    W1 = Register_REG_W1 as u32,
    W2 = Register_REG_W2 as u32,
    W3 = Register_REG_W3 as u32,
    W4 = Register_REG_W4 as u32,
    W5 = Register_REG_W5 as u32,
    W6 = Register_REG_W6 as u32,
    W7 = Register_REG_W7 as u32,
    W8 = Register_REG_W8 as u32,
    W9 = Register_REG_W9 as u32,
    W10 = Register_REG_W10 as u32,
    W11 = Register_REG_W11 as u32,
    W12 = Register_REG_W12 as u32,
    W13 = Register_REG_W13 as u32,
    W14 = Register_REG_W14 as u32,
    W15 = Register_REG_W15 as u32,
    W16 = Register_REG_W16 as u32,
    W17 = Register_REG_W17 as u32,
    W18 = Register_REG_W18 as u32,
    W19 = Register_REG_W19 as u32,
    W20 = Register_REG_W20 as u32,
    W21 = Register_REG_W21 as u32,
    W22 = Register_REG_W22 as u32,
    W23 = Register_REG_W23 as u32,
    W24 = Register_REG_W24 as u32,
    W25 = Register_REG_W25 as u32,
    W26 = Register_REG_W26 as u32,
    W27 = Register_REG_W27 as u32,
    W28 = Register_REG_W28 as u32,
    W29 = Register_REG_W29 as u32,
    W30 = Register_REG_W30 as u32,
    WZR = Register_REG_WZR as u32,
    WSP = Register_REG_WSP as u32,
    X0 = Register_REG_X0 as u32,
    X1 = Register_REG_X1 as u32,
    X2 = Register_REG_X2 as u32,
    X3 = Register_REG_X3 as u32,
    X4 = Register_REG_X4 as u32,
    X5 = Register_REG_X5 as u32,
    X6 = Register_REG_X6 as u32,
    X7 = Register_REG_X7 as u32,
    X8 = Register_REG_X8 as u32,
    X9 = Register_REG_X9 as u32,
    X10 = Register_REG_X10 as u32,
    X11 = Register_REG_X11 as u32,
    X12 = Register_REG_X12 as u32,
    X13 = Register_REG_X13 as u32,
    X14 = Register_REG_X14 as u32,
    X15 = Register_REG_X15 as u32,
    X16 = Register_REG_X16 as u32,
    X17 = Register_REG_X17 as u32,
    X18 = Register_REG_X18 as u32,
    X19 = Register_REG_X19 as u32,
    X20 = Register_REG_X20 as u32,
    X21 = Register_REG_X21 as u32,
    X22 = Register_REG_X22 as u32,
    X23 = Register_REG_X23 as u32,
    X24 = Register_REG_X24 as u32,
    X25 = Register_REG_X25 as u32,
    X26 = Register_REG_X26 as u32,
    X27 = Register_REG_X27 as u32,
    X28 = Register_REG_X28 as u32,
    X29 = Register_REG_X29 as u32,
    X30 = Register_REG_X30 as u32,
    XZR = Register_REG_XZR as u32,
    SP = Register_REG_SP as u32,
    V0 = Register_REG_V0 as u32,
    V1 = Register_REG_V1 as u32,
    V2 = Register_REG_V2 as u32,
    V3 = Register_REG_V3 as u32,
    V4 = Register_REG_V4 as u32,
    V5 = Register_REG_V5 as u32,
    V6 = Register_REG_V6 as u32,
    V7 = Register_REG_V7 as u32,
    V8 = Register_REG_V8 as u32,
    V9 = Register_REG_V9 as u32,
    V10 = Register_REG_V10 as u32,
    V11 = Register_REG_V11 as u32,
    V12 = Register_REG_V12 as u32,
    V13 = Register_REG_V13 as u32,
    V14 = Register_REG_V14 as u32,
    V15 = Register_REG_V15 as u32,
    V16 = Register_REG_V16 as u32,
    V17 = Register_REG_V17 as u32,
    V18 = Register_REG_V18 as u32,
    V19 = Register_REG_V19 as u32,
    V20 = Register_REG_V20 as u32,
    V21 = Register_REG_V21 as u32,
    V22 = Register_REG_V22 as u32,
    V23 = Register_REG_V23 as u32,
    V24 = Register_REG_V24 as u32,
    V25 = Register_REG_V25 as u32,
    V26 = Register_REG_V26 as u32,
    V27 = Register_REG_V27 as u32,
    V28 = Register_REG_V28 as u32,
    V29 = Register_REG_V29 as u32,
    V30 = Register_REG_V30 as u32,
    V31 = Register_REG_V31 as u32,
    B0 = Register_REG_B0 as u32,
    B1 = Register_REG_B1 as u32,
    B2 = Register_REG_B2 as u32,
    B3 = Register_REG_B3 as u32,
    B4 = Register_REG_B4 as u32,
    B5 = Register_REG_B5 as u32,
    B6 = Register_REG_B6 as u32,
    B7 = Register_REG_B7 as u32,
    B8 = Register_REG_B8 as u32,
    B9 = Register_REG_B9 as u32,
    B10 = Register_REG_B10 as u32,
    B11 = Register_REG_B11 as u32,
    B12 = Register_REG_B12 as u32,
    B13 = Register_REG_B13 as u32,
    B14 = Register_REG_B14 as u32,
    B15 = Register_REG_B15 as u32,
    B16 = Register_REG_B16 as u32,
    B17 = Register_REG_B17 as u32,
    B18 = Register_REG_B18 as u32,
    B19 = Register_REG_B19 as u32,
    B20 = Register_REG_B20 as u32,
    B21 = Register_REG_B21 as u32,
    B22 = Register_REG_B22 as u32,
    B23 = Register_REG_B23 as u32,
    B24 = Register_REG_B24 as u32,
    B25 = Register_REG_B25 as u32,
    B26 = Register_REG_B26 as u32,
    B27 = Register_REG_B27 as u32,
    B28 = Register_REG_B28 as u32,
    B29 = Register_REG_B29 as u32,
    B30 = Register_REG_B30 as u32,
    B31 = Register_REG_B31 as u32,
    H0 = Register_REG_H0 as u32,
    H1 = Register_REG_H1 as u32,
    H2 = Register_REG_H2 as u32,
    H3 = Register_REG_H3 as u32,
    H4 = Register_REG_H4 as u32,
    H5 = Register_REG_H5 as u32,
    H6 = Register_REG_H6 as u32,
    H7 = Register_REG_H7 as u32,
    H8 = Register_REG_H8 as u32,
    H9 = Register_REG_H9 as u32,
    H10 = Register_REG_H10 as u32,
    H11 = Register_REG_H11 as u32,
    H12 = Register_REG_H12 as u32,
    H13 = Register_REG_H13 as u32,
    H14 = Register_REG_H14 as u32,
    H15 = Register_REG_H15 as u32,
    H16 = Register_REG_H16 as u32,
    H17 = Register_REG_H17 as u32,
    H18 = Register_REG_H18 as u32,
    H19 = Register_REG_H19 as u32,
    H20 = Register_REG_H20 as u32,
    H21 = Register_REG_H21 as u32,
    H22 = Register_REG_H22 as u32,
    H23 = Register_REG_H23 as u32,
    H24 = Register_REG_H24 as u32,
    H25 = Register_REG_H25 as u32,
    H26 = Register_REG_H26 as u32,
    H27 = Register_REG_H27 as u32,
    H28 = Register_REG_H28 as u32,
    H29 = Register_REG_H29 as u32,
    H30 = Register_REG_H30 as u32,
    H31 = Register_REG_H31 as u32,
    S0 = Register_REG_S0 as u32,
    S1 = Register_REG_S1 as u32,
    S2 = Register_REG_S2 as u32,
    S3 = Register_REG_S3 as u32,
    S4 = Register_REG_S4 as u32,
    S5 = Register_REG_S5 as u32,
    S6 = Register_REG_S6 as u32,
    S7 = Register_REG_S7 as u32,
    S8 = Register_REG_S8 as u32,
    S9 = Register_REG_S9 as u32,
    S10 = Register_REG_S10 as u32,
    S11 = Register_REG_S11 as u32,
    S12 = Register_REG_S12 as u32,
    S13 = Register_REG_S13 as u32,
    S14 = Register_REG_S14 as u32,
    S15 = Register_REG_S15 as u32,
    S16 = Register_REG_S16 as u32,
    S17 = Register_REG_S17 as u32,
    S18 = Register_REG_S18 as u32,
    S19 = Register_REG_S19 as u32,
    S20 = Register_REG_S20 as u32,
    S21 = Register_REG_S21 as u32,
    S22 = Register_REG_S22 as u32,
    S23 = Register_REG_S23 as u32,
    S24 = Register_REG_S24 as u32,
    S25 = Register_REG_S25 as u32,
    S26 = Register_REG_S26 as u32,
    S27 = Register_REG_S27 as u32,
    S28 = Register_REG_S28 as u32,
    S29 = Register_REG_S29 as u32,
    S30 = Register_REG_S30 as u32,
    S31 = Register_REG_S31 as u32,
    D0 = Register_REG_D0 as u32,
    D1 = Register_REG_D1 as u32,
    D2 = Register_REG_D2 as u32,
    D3 = Register_REG_D3 as u32,
    D4 = Register_REG_D4 as u32,
    D5 = Register_REG_D5 as u32,
    D6 = Register_REG_D6 as u32,
    D7 = Register_REG_D7 as u32,
    D8 = Register_REG_D8 as u32,
    D9 = Register_REG_D9 as u32,
    D10 = Register_REG_D10 as u32,
    D11 = Register_REG_D11 as u32,
    D12 = Register_REG_D12 as u32,
    D13 = Register_REG_D13 as u32,
    D14 = Register_REG_D14 as u32,
    D15 = Register_REG_D15 as u32,
    D16 = Register_REG_D16 as u32,
    D17 = Register_REG_D17 as u32,
    D18 = Register_REG_D18 as u32,
    D19 = Register_REG_D19 as u32,
    D20 = Register_REG_D20 as u32,
    D21 = Register_REG_D21 as u32,
    D22 = Register_REG_D22 as u32,
    D23 = Register_REG_D23 as u32,
    D24 = Register_REG_D24 as u32,
    D25 = Register_REG_D25 as u32,
    D26 = Register_REG_D26 as u32,
    D27 = Register_REG_D27 as u32,
    D28 = Register_REG_D28 as u32,
    D29 = Register_REG_D29 as u32,
    D30 = Register_REG_D30 as u32,
    D31 = Register_REG_D31 as u32,
    Q0 = Register_REG_Q0 as u32,
    Q1 = Register_REG_Q1 as u32,
    Q2 = Register_REG_Q2 as u32,
    Q3 = Register_REG_Q3 as u32,
    Q4 = Register_REG_Q4 as u32,
    Q5 = Register_REG_Q5 as u32,
    Q6 = Register_REG_Q6 as u32,
    Q7 = Register_REG_Q7 as u32,
    Q8 = Register_REG_Q8 as u32,
    Q9 = Register_REG_Q9 as u32,
    Q10 = Register_REG_Q10 as u32,
    Q11 = Register_REG_Q11 as u32,
    Q12 = Register_REG_Q12 as u32,
    Q13 = Register_REG_Q13 as u32,
    Q14 = Register_REG_Q14 as u32,
    Q15 = Register_REG_Q15 as u32,
    Q16 = Register_REG_Q16 as u32,
    Q17 = Register_REG_Q17 as u32,
    Q18 = Register_REG_Q18 as u32,
    Q19 = Register_REG_Q19 as u32,
    Q20 = Register_REG_Q20 as u32,
    Q21 = Register_REG_Q21 as u32,
    Q22 = Register_REG_Q22 as u32,
    Q23 = Register_REG_Q23 as u32,
    Q24 = Register_REG_Q24 as u32,
    Q25 = Register_REG_Q25 as u32,
    Q26 = Register_REG_Q26 as u32,
    Q27 = Register_REG_Q27 as u32,
    Q28 = Register_REG_Q28 as u32,
    Q29 = Register_REG_Q29 as u32,
    Q30 = Register_REG_Q30 as u32,
    Q31 = Register_REG_Q31 as u32,
    Z0 = Register_REG_Z0 as u32,
    Z1 = Register_REG_Z1 as u32,
    Z2 = Register_REG_Z2 as u32,
    Z3 = Register_REG_Z3 as u32,
    Z4 = Register_REG_Z4 as u32,
    Z5 = Register_REG_Z5 as u32,
    Z6 = Register_REG_Z6 as u32,
    Z7 = Register_REG_Z7 as u32,
    Z8 = Register_REG_Z8 as u32,
    Z9 = Register_REG_Z9 as u32,
    Z10 = Register_REG_Z10 as u32,
    Z11 = Register_REG_Z11 as u32,
    Z12 = Register_REG_Z12 as u32,
    Z13 = Register_REG_Z13 as u32,
    Z14 = Register_REG_Z14 as u32,
    Z15 = Register_REG_Z15 as u32,
    Z16 = Register_REG_Z16 as u32,
    Z17 = Register_REG_Z17 as u32,
    Z18 = Register_REG_Z18 as u32,
    Z19 = Register_REG_Z19 as u32,
    Z20 = Register_REG_Z20 as u32,
    Z21 = Register_REG_Z21 as u32,
    Z22 = Register_REG_Z22 as u32,
    Z23 = Register_REG_Z23 as u32,
    Z24 = Register_REG_Z24 as u32,
    Z25 = Register_REG_Z25 as u32,
    Z26 = Register_REG_Z26 as u32,
    Z27 = Register_REG_Z27 as u32,
    Z28 = Register_REG_Z28 as u32,
    Z29 = Register_REG_Z29 as u32,
    Z30 = Register_REG_Z30 as u32,
    Z31 = Register_REG_Z31 as u32,
    P0 = Register_REG_P0 as u32,
    P1 = Register_REG_P1 as u32,
    P2 = Register_REG_P2 as u32,
    P3 = Register_REG_P3 as u32,
    P4 = Register_REG_P4 as u32,
    P5 = Register_REG_P5 as u32,
    P6 = Register_REG_P6 as u32,
    P7 = Register_REG_P7 as u32,
    P8 = Register_REG_P8 as u32,
    P9 = Register_REG_P9 as u32,
    P10 = Register_REG_P10 as u32,
    P11 = Register_REG_P11 as u32,
    P12 = Register_REG_P12 as u32,
    P13 = Register_REG_P13 as u32,
    P14 = Register_REG_P14 as u32,
    P15 = Register_REG_P15 as u32,
    P16 = Register_REG_P16 as u32,
    P17 = Register_REG_P17 as u32,
    P18 = Register_REG_P18 as u32,
    P19 = Register_REG_P19 as u32,
    P20 = Register_REG_P20 as u32,
    P21 = Register_REG_P21 as u32,
    P22 = Register_REG_P22 as u32,
    P23 = Register_REG_P23 as u32,
    P24 = Register_REG_P24 as u32,
    P25 = Register_REG_P25 as u32,
    P26 = Register_REG_P26 as u32,
    P27 = Register_REG_P27 as u32,
    P28 = Register_REG_P28 as u32,
    P29 = Register_REG_P29 as u32,
    P30 = Register_REG_P30 as u32,
    P31 = Register_REG_P31 as u32,
    ZT0 = Register_REG_ZT0 as u32,
}

const_assert_eq!(Register_REG_END, Register_REG_ZT0 + 1);

impl Reg {
    /// Returns the register name
    ///
    /// # Examples
    /// ```
    /// use bad64::Reg;
    ///
    /// assert_eq!(Reg::X0.name(), "x0");
    /// ```
    pub fn name(&self) -> &'static str {
        #[cfg(target_os = "windows")]
        {
            unsafe { CStr::from_ptr(get_register_name(self.to_i32().unwrap()) as _) }
                .to_str()
                .unwrap()
        }
        #[cfg(not(target_os = "windows"))]
        {
            unsafe { CStr::from_ptr(get_register_name(self.to_u32().unwrap()) as _) }
                .to_str()
                .unwrap()
        }
    }

    /// Returns the register size
    ///
    /// # Examples
    /// ```
    /// use bad64::Reg;
    ///
    /// assert_eq!(Reg::X0.size(), 8);
    /// assert_eq!(Reg::V0.size(), 16);
    /// ```
    ///
    /// ```
    /// use bad64::{decode, Operand, Reg};
    ///
    /// // add x0, x1, #0x41  - "\x20\x04\x01\x91"
    /// let decoded = decode(0x91010420, 0x1000).unwrap();
    ///
    /// let op = decoded.operands()[0];
    ///
    /// assert_eq!(op, Operand::Reg { reg: Reg::X0, arrspec: None });
    ///
    /// match op {
    ///     Operand::Reg { reg: r, .. } => assert_eq!(r.size(), 8),
    ///     _ => assert!(false),
    /// };
    /// ```
    pub fn size(&self) -> usize {
        #[cfg(target_os = "windows")]
        {
            unsafe { bad64_sys::get_register_size(self.to_i32().unwrap()) as usize }
        }
        #[cfg(not(target_os = "windows"))]
        {
            unsafe { bad64_sys::get_register_size(self.to_u32().unwrap()) as usize }
        }
    }

    /// Returns register's SIMD status
    ///
    /// # Example
    /// ```
    /// use bad64::Reg;
    ///
    /// assert_eq!(Reg::V0.is_simd(), true);
    /// assert_eq!(Reg::D0.is_simd(), false);
    /// assert_eq!(Reg::X0.is_simd(), false);
    /// assert_eq!(Reg::Z0.is_simd(), false);
    /// assert_eq!(Reg::P0.is_simd(), false);
    /// ```
    pub fn is_simd(&self) -> bool {
        self.to_u32().unwrap() >= Reg::V0.to_u32().unwrap()
            && self.to_u32().unwrap() <= Reg::V31.to_u32().unwrap()
    }

    /// Returns register's SVE status
    ///
    /// # Example
    /// ```
    /// use bad64::Reg;
    ///
    /// assert_eq!(Reg::Z0.is_sve(), true);
    /// assert_eq!(Reg::V0.is_sve(), false);
    /// assert_eq!(Reg::D0.is_sve(), false);
    /// assert_eq!(Reg::X0.is_sve(), false);
    /// assert_eq!(Reg::P0.is_simd(), false);
    /// ```
    pub fn is_sve(&self) -> bool {
        self.to_u32().unwrap() >= Reg::Z0.to_u32().unwrap()
            && self.to_u32().unwrap() <= Reg::Z31.to_u32().unwrap()
    }

    /// Returns register's predicate status
    ///
    /// # Example
    /// ```
    /// use bad64::Reg;
    ///
    /// assert_eq!(Reg::P0.is_pred(), true);
    /// assert_eq!(Reg::V0.is_pred(), false);
    /// assert_eq!(Reg::D0.is_pred(), false);
    /// assert_eq!(Reg::X0.is_pred(), false);
    /// assert_eq!(Reg::Z0.is_pred(), false);
    /// ```
    pub fn is_pred(&self) -> bool {
        self.to_u32().unwrap() >= Reg::P0.to_u32().unwrap()
            && self.to_u32().unwrap() <= Reg::P31.to_u32().unwrap()
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
