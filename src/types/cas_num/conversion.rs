//for converting primitives to CASNum and vice versa

use core::{f32, num};
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

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        };
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

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        };
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

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        };
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

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        };
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

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: Sign::Pos,
        };
    }
}

impl From<i8> for CASNum {
    fn from(value: i8) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.abs() as DigitType);
        }

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i16> for CASNum {
    fn from(value: i16) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.abs() as DigitType);
        }

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i32> for CASNum {
    fn from(value: i32) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.abs() as DigitType);
        }

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i64> for CASNum {
    fn from(value: i64) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        if value != 0 {
            digits.push_back(value.abs() as DigitType);
        }

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i128> for CASNum {
    fn from(mut value: i128) -> Self {
        let mut digits: VecDeque<DigitType> = VecDeque::new();

        let mut abs = value.abs();

        while abs > 0 {
            digits.push_back((abs & 0xFFFFFFFFFFFFFFFF).try_into().unwrap());
            abs >>= 64;
        }

        return CASNum {
            value: CASValue::Finite { digits, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
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
<<<<<<< HEAD

=======
>>>>>>> 53fd65d (Changed CASValue to use 64 bit ints instead of 8 bit)
        let mut digits: VecDeque<DigitType> = VecDeque::new();
        let bits = value.to_bits();
        const SIGN_MASK: u32 = 0x80000000;
        const MANTISSA_MASK: u32 = 0x007FFFFF;
        const MANTISSA_SIZE: i128 = 23;
        let sign: Sign = if bits & SIGN_MASK == 0 {
            Sign::Pos
        } else {
            Sign::Neg
        };
<<<<<<< HEAD
        let mut exp: i128 = i128::from((bits >> MANTISSA_SIZE) & 0xff) - 127 - MANTISSA_SIZE;
        let mantissa: DigitType = DigitType::from(bits & MANTISSA_MASK) + 0x800000;
        //fp values are 1.(mantissa) * 2^exp * (-1)^sign
        //so we add the 1 back in

        let mantissa_lower;
        let mantissa_higher;

        if exp > 0 {
            //we have to split mantissa in half if it straddles the boundary
            let exp_rem = exp % NUM_BITS as i128;
            mantissa_lower = mantissa << exp_rem;
            let mantissa_higher_mask: DigitType =
                ((1 << exp_rem) - 1) << (NUM_BITS as i128 - exp_rem);
            //bit mask of exp_rem 1s to extract highest exp_rem bits from mantissa
            mantissa_higher = (mantissa & mantissa_higher_mask) >> (NUM_BITS - exp_rem);

            if mantissa_higher == 0 {
                digits.push_back(mantissa_lower);
            } else if mantissa_lower == 0 {
                digits.push_back(mantissa_higher);
                exp += NUM_BITS;
            } else {
                digits.push_front(mantissa_higher);
                digits.push_front(mantissa_lower);
            }
        } else {
            let exp_rem = (-exp) % NUM_BITS as i128;
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
                exp -= 2 * NUM_BITS;
            }
        }

        let num_digits = digits.len();
=======
        let mut exp: i64 = i64::from((bits >> 23) & 255) - 127 - 23;
        let mantissa: DigitType = ((bits & MANTISSA_MASK) + 0x00800000) as DigitType;
        //fp values are 1.(mantissa) * 2^exp * (-1)^sign
        //so we add the 1 back in

        let mantissa_lower: DigitType;
        let mantissa_higher: DigitType;

        if exp > 0 {
            //we have to split mantissa in half if it straddles the boundary
            let exp_rem = exp % NUM_BITS as i64;
            mantissa_lower = mantissa << exp_rem;
            let mantissa_higher_mask: DigitType =
                ((1 << exp_rem) - 1) << (NUM_BITS as i64 - exp_rem);
            //bit mask of exp_rem 1s to extract highest exp_rem bits from mantissa
            mantissa_higher = (mantissa & mantissa_higher_mask) >> (NUM_BITS as i64 - exp_rem);
        } else {
            let exp_rem = (-exp) % NUM_BITS as i64;
            mantissa_higher = mantissa >> exp_rem;
            let mantissa_lower_mask: DigitType = (1 << exp_rem) - 1;
            mantissa_lower = (mantissa & mantissa_lower_mask) << (NUM_BITS as i64 - exp_rem);
        }

        if mantissa_higher == 0 {
            digits.push_back(mantissa_lower);
        } else if mantissa_lower == 0 {
            digits.push_back(mantissa_higher);
            exp += 1;
        } else {
            digits.push_front(mantissa_higher);
            digits.push_front(mantissa_lower);
        }
>>>>>>> 53fd65d (Changed CASValue to use 64 bit ints instead of 8 bit)

        return CASNum {
            value: CASValue::Finite {
                digits,
<<<<<<< HEAD
                exp: ((exp / NUM_BITS) + (num_digits as i128) - 1) as isize,
=======
                exp: ((exp / 128) as isize),
>>>>>>> 53fd65d (Changed CASValue to use 64 bit ints instead of 8 bit)
            }
            .normalize(),
            sign,
        };
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
        let mut exp: i128 = i128::from((bits >> 52) & 0x7ff) - 1023 - 52;
        let mantissa: DigitType = DigitType::from(bits & MANTISSA_MASK) + 0x10000000000000;
        //fp values are 1.(mantissa) * 2^exp * (-1)^sign
        //so we add the 1 back in

        let mantissa_lower;
        let mantissa_higher;

        if exp > 0 {
            //we have to split mantissa in half if it straddles the boundary
            let exp_rem = exp % NUM_BITS as i128;
            mantissa_lower = mantissa << exp_rem;
            let mantissa_higher_mask: DigitType =
                ((1 << exp_rem) - 1) << (NUM_BITS as i128 - exp_rem);
            //bit mask of exp_rem 1s to extract highest exp_rem bits from mantissa
            mantissa_higher = (mantissa & mantissa_higher_mask) >> (NUM_BITS - exp_rem);

            if mantissa_higher == 0 {
                digits.push_back(mantissa_lower);
            } else if mantissa_lower == 0 {
                digits.push_back(mantissa_higher);
                exp += NUM_BITS;
            } else {
                digits.push_front(mantissa_higher);
                digits.push_front(mantissa_lower);
            }
        } else {
            let exp_rem = (-exp) % NUM_BITS as i128;
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
                exp -= 2 * NUM_BITS;
            }
        }

        let num_digits = digits.len();

        return CASNum {
            value: CASValue::Finite {
                digits,
                exp: ((exp / NUM_BITS) + (num_digits as i128) - 1) as isize,
            }
            .normalize(),
            sign,
        };
    }
}

// impl Into<u8> for CASNum {
//     fn into(self) -> u8 {
//         let min_digit = self.exp;
//         if min_digit > 0 {
//             //no ones place
//             return 0;
//         } else {
//             return match self.digits.get((-min_digit).try_into().unwrap()) {
//                 Some(x) => *x,
//                 None => 0,
//             };
//         }
//     }
// }

// impl Into<u16> for CASNum {
//     fn into(self) -> u16 {
//         todo!()
//     }
// }

// impl Into<u32> for CASNum {
//     fn into(self) -> u32 {
//         todo!()
//     }
// }

// impl Into<u64> for CASNum {
//     fn into(self) -> u64 {
//         todo!()
//     }
// }

// impl Into<u128> for CASNum {
//     fn into(self) -> u128 {
//         todo!()
//     }
// }

// impl Into<i8> for CASNum {
//     fn into(self) -> i8 {
//         todo!()
//     }
// }

// impl Into<i16> for CASNum {
//     fn into(self) -> i16 {
//         todo!()
//     }
// }

// impl Into<i32> for CASNum {
//     fn into(self) -> i32 {
//         todo!()
//     }
// }

// impl Into<i64> for CASNum {
//     fn into(self) -> i64 {
//         todo!()
//     }
// }

// impl Into<i128> for CASNum {
//     fn into(self) -> i128 {
//         todo!()
//     }
// }

// impl Into<f32> for CASNum {
//     fn into(self) -> f32 {
//         todo!()
//     }
// }

// impl Into<f64> for CASNum {
//     fn into(self) -> f64 {
//         match self {
//             Self {
//                 value: CASValue::Finite { digits, exp },
//                 sign,
//             } => {
//                 let sign: u64 = if sign == Sign::Pos {
//                     0x8000000000000000
//                 } else {
//                     0x0
//                 };
//                 let mut exponent: u64 = (exp * 8 + 1023 + 52) as u64 + (digits.len() as u64);
//                 let mut mantissa: Vec<&u8> = digits.iter().rev().take(6).collect();
//                 while mantissa.len() < 7 {
//                     exponent -= 8;
//                     mantissa.push(&0);
//                 }

//                 let mut mantissa_bits: u64 = 0;
//                 for num in mantissa {
//                     mantissa_bits += *num as u64; //concatenate digits
//                     mantissa_bits <<= 8;
//                 }

//                 mantissa_bits &= 0x00FFFFFFFFFFFFFF; //get rid of trailing 1
//                 mantissa_bits >>= 4; //get it from 56 to 52 bits

//                 return f64::from_bits(sign | (exponent << 52) | mantissa_bits);
//             }
//             Self {
//                 value: CASValue::Indeterminate,
//                 ..
//             } => return f64::NAN,
//             Self {
//                 value: CASValue::Infinite,
//                 sign: Sign::Pos,
//             } => return f64::INFINITY,
//             Self {
//                 value: CASValue::Infinite,
//                 sign: Sign::Neg,
//             } => return f64::NEG_INFINITY,
//         }
//     }
// }
