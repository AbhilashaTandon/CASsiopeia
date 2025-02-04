//for converting primitives to CASNum and vice versa

use core::f32;
use std::collections::VecDeque;

use crate::types::cas_num::{DigitType, NUM_BITS};

use super::{CASNum, CASValue, Sign};

impl From<u8> for CASNum {
    fn from(value: u8) -> Self {
        //we can avoid a normalize call since 0 is the only case where normalization is necessary
        //which will give an empty vecdeque

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        if value != 0 {
            digits.push_back(value as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        }
    }
}

impl From<u16> for CASNum {
    fn from(value: u16) -> Self {
        //we can avoid a normalize call since 0 is the only case where normalization is necessary
        //which will give an empty vecdeque

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        if value != 0 {
            digits.push_back(value as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        }
    }
}

impl From<u32> for CASNum {
    fn from(value: u32) -> Self {
        //we can avoid a normalize call since 0 is the only case where normalization is necessary
        //which will give an empty vecdeque

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        if value != 0 {
            digits.push_back(value as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        }
    }
}

impl From<u64> for CASNum {
    fn from(value: u64) -> Self {
        //we can avoid a normalize call since 0 is the only case where normalization is necessary
        //which will give an empty vecdeque

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        if value != 0 {
            digits.push_back(value as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        }
    }
}

impl From<u128> for CASNum {
    fn from(value: u128) -> Self {
        //we can avoid a normalize call since 0 is the only case where normalization is necessary
        //which will give an empty vecdeque

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        if value != 0 {
            digits.push_back(value as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        }
    }
}

impl From<i8> for CASNum {
    fn from(value: i8) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.unsigned_abs() as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        }
    }
}

impl From<i16> for CASNum {
    fn from(value: i16) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.unsigned_abs() as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        }
    }
}

impl From<i32> for CASNum {
    fn from(value: i32) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.unsigned_abs() as DigitType);
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        }
    }
}

impl From<i64> for CASNum {
    fn from(value: i64) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.unsigned_abs());
        }

        CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        }
    }
}

impl From<i128> for CASNum {
    fn from(value: i128) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        let mut abs = value.abs();

        let mut exp: isize = -1;
        while abs > 0 {
            digits.push_back((abs & 0xFFFFFFFFFFFFFFFF).try_into().unwrap());
            abs >>= 64;
            exp += 1;
        }

        CASNum {
            value: CASValue::Finite { digits, exp },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        }
    }
}

impl From<f32> for CASNum {
    fn from(value: f32) -> CASNum
    where
        Self: Sized,
    {
        if value.is_nan() {
            return CASNum {
                value: CASValue::Indeterminate,
                sign: Sign::Pos,
            };
        }
        match value {
            f32::INFINITY => {
                return CASNum {
                    value: CASValue::Infinite,
                    sign: Sign::Pos,
                }
            }
            f32::NEG_INFINITY => {
                return CASNum {
                    value: CASValue::Infinite,
                    sign: Sign::Neg,
                }
            }

            0.0 => {
                //also matches -0 but that doesn't really matter
                return CASNum {
                    value: CASValue::Finite {
                        digits: VecDeque::new(),
                        exp: 0,
                    },
                    sign: Sign::Pos,
                };
            }
            _ => {}
        }

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        let bits = value.to_bits();
        const SIGN_MASK: u32 = 0x80000000;
        const MANTISSA_MASK: u32 = 0x7FFFFF;
        const MANTISSA_BITS: i128 = 23;
        let sign: Sign = if bits & SIGN_MASK == 0 {
            Sign::Pos
        } else {
            Sign::Neg
        };
        let mut exp: i128 = i128::from((bits >> MANTISSA_BITS) & 0xFF) - 127 - 23;
        let mantissa: DigitType = DigitType::from(bits & MANTISSA_MASK) + 0x800000;
        //fp values are 1.(mantissa) * 2^exp * (-1)^sign
        //so we add the 1 back in

        let mantissa_lower;
        let mantissa_higher;

        if exp >= 0 {
            //we have to split mantissa in half if it straddles the boundary
            let exp_rem = exp % NUM_BITS;
            mantissa_lower = mantissa << exp_rem;
            let mantissa_higher_mask: DigitType =
                ((1 << exp_rem) - 1) << (NUM_BITS - exp_rem);
            //bit mask of exp_rem 1s to extract highest exp_rem bits from mantissa
            mantissa_higher = (mantissa & mantissa_higher_mask) >> (NUM_BITS - exp_rem);

            if mantissa_higher == 0 {
                digits.push_back(mantissa_lower);
            } else if mantissa_lower == 0 {
                digits.push_back(mantissa_higher);
            } else {
                digits.push_front(mantissa_higher);
                digits.push_front(mantissa_lower);
            }
        } else {
            let exp_rem = (-exp) % NUM_BITS;
            mantissa_higher = mantissa >> exp_rem;
            let mantissa_lower_mask: DigitType = (1 << exp_rem) - 1;
            mantissa_lower = (mantissa & mantissa_lower_mask) << (NUM_BITS - exp_rem);

            if mantissa_higher == 0 {
                digits.push_back(mantissa_lower);
                exp -= NUM_BITS;
            } else if mantissa_lower == 0 {
                digits.push_back(mantissa_higher);
            } else {
                digits.push_front(mantissa_higher);
                digits.push_front(mantissa_lower);
                // exp -= NUM_BITS;
            }
        }

        // let num_digits = digits.len();

        CASNum {
            value: CASValue::Finite {
                digits,
                exp: (exp / NUM_BITS) as isize,
            }
            .normalize(),
            sign,
        }
    }
}

impl From<f64> for CASNum {
    fn from(value: f64) -> Self {
        if value.is_nan() {
            return CASNum {
                value: CASValue::Indeterminate,
                sign: Sign::Pos,
            };
        }
        match value {
            f64::INFINITY => {
                return CASNum {
                    value: CASValue::Infinite,
                    sign: Sign::Pos,
                }
            }
            f64::NEG_INFINITY => {
                return CASNum {
                    value: CASValue::Infinite,
                    sign: Sign::Neg,
                }
            }

            0.0 => {
                //also matches -0 but that doesn't really matter
                return CASNum {
                    value: CASValue::Finite {
                        digits: VecDeque::new(),
                        exp: 0,
                    },
                    sign: Sign::Pos,
                };
            }
            _ => {}
        }

        let mut digits: VecDeque<DigitType> = VecDeque::new();
        let bits = value.to_bits();

        const SIGN_MASK: u64 = 0x8000000000000000;
        const MANTISSA_MASK: u64 = 0x000FFFFFFFFFFFFF;
        let sign: Sign = if bits & SIGN_MASK == 0 {
            Sign::Pos
        } else {
            Sign::Neg
        };
        let raw_exp = ((bits >> 52) & 0x7ff) as i128;
        let raw_mantissa = (bits & MANTISSA_MASK) as DigitType;

        let mut exp: i128 = raw_exp - 1023 - 52;
        let mantissa: DigitType = raw_mantissa + 0x10000000000000;
        //fp values are 1.(mantissa) * 2^exp * (-1)^sign
        //so we add the 1 back in

        let mantissa_lower;
        let mantissa_higher;

        if exp > 0 {
            //we have to split mantissa in half if it straddles the boundary
            let exp_rem = exp % NUM_BITS;
            mantissa_lower = mantissa << exp_rem;
            if exp_rem != 0 {
                let mantissa_higher_mask: DigitType =
                    ((1 << exp_rem) - 1) << (NUM_BITS - exp_rem);
                //bit mask of exp_rem 1s to extract highest exp_rem bits from mantissa
                mantissa_higher = (mantissa & mantissa_higher_mask) >> (NUM_BITS - exp_rem);
            } else {
                mantissa_higher = 0;
            }

            if mantissa_higher == 0 {
                digits.push_back(mantissa_lower);
            } else if mantissa_lower == 0 {
                digits.push_back(mantissa_higher);
                exp += NUM_BITS;
            } else {
                digits.push_front(mantissa_higher);
                digits.push_front(mantissa_lower);
                exp += NUM_BITS;
            }
        } else {
            let exp_rem = (-exp) % NUM_BITS;
            mantissa_higher = mantissa >> exp_rem;
            let mantissa_lower_mask: DigitType = (1 << exp_rem) - 1;
            if exp_rem == 0 {
                mantissa_lower = 0;
            } else {
                mantissa_lower = (mantissa & mantissa_lower_mask) << (NUM_BITS - exp_rem);
            }

            if mantissa_higher == 0 {
                digits.push_back(mantissa_lower);
                exp -= NUM_BITS;
            } else if mantissa_lower == 0 {
                digits.push_back(mantissa_higher);
            } else {
                digits.push_front(mantissa_higher);
                digits.push_front(mantissa_lower);
            }
        }

        CASNum {
            value: CASValue::Finite {
                digits,
                exp: (exp / NUM_BITS) as isize,
            }
            .normalize(),
            sign,
        }
    }
}

impl From<CASNum> for f64 {
    fn from(val: CASNum) -> Self {
        //
        if val.value.is_zero() {
            return 0.;
        }
        match val {
            CASNum {
                value: CASValue::Indeterminate,
                ..
            } => f64::NAN,
            CASNum {
                value: CASValue::Infinite,
                sign: Sign::Pos,
            } => f64::INFINITY,
            CASNum {
                value: CASValue::Infinite,
                sign: Sign::Neg,
            } => f64::NEG_INFINITY,
            CASNum {
                value: CASValue::Finite { mut digits, exp },
                sign,
            } => {
                let sign: u64 = if sign == Sign::Neg {
                    1_u64 << 63
                } else {
                    0
                };

                let mut exponent: i64 = (exp * 64 + 52) as i64;

                let mut higher_digit: u64 = digits.pop_back().unwrap();
                //we can safely unwrap since digits is only empty if self == 0

                let mut first_1: i64 = 0;
                //index starting at 1 of first 1 in bits of higher_digit

                while (higher_digit >> first_1) > 0 {
                    first_1 += 1;
                    if first_1 >= 64 {
                        //avoid overflow
                        first_1 = 64;
                        break;
                    }
                }

                if first_1 < 53 {
                    let lower_digit: Option<u64> = digits.pop_back();
                    //if has less than 53 bits we need to extend with next digit

                    higher_digit <<= 53_i64 - first_1; //should make first 1 have position 53
                                                          //has first_1 digits
                    if let Some(mut bits) = lower_digit {
                        bits >>= 64 - (53 - first_1);
                        higher_digit |= bits;
                        exponent -= 64; //i have no idea why this works
                    } else {
                        exponent -= 64; //i have no idea why this works
                    }

                    exponent += 64 - (53 - first_1);
                } else {
                    //if has 53 or more we need to get rid of excess bits
                    higher_digit >>= first_1 - 53;

                    exponent += first_1 - 53;
                }

                let mantissa = higher_digit & 0xFFFFFFFFFFFFF; //get rid of leading 1

                f64::from_bits(sign | (((exponent + 1023) as u64) << 52) | mantissa)
            }
        }
    }
}
