//for converting primitives to CASNum and vice versa

use core::f32;
use std::collections::VecDeque;

use super::{CASNum, CASValue, Sign};

impl From<u8> for CASNum {
    fn from(value: u8) -> Self {
        //we can avoid a normalize call since 0 is the only case where normalization is necessary
        //which will give an empty vecdeque

        let mut bytes: VecDeque<u8> = VecDeque::new();
        if value != 0 {
            bytes.push_back(value);
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 },
            sign: Sign::Pos,
        };
    }
}

impl From<u16> for CASNum {
    fn from(value: u16) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255) as u8;
            bytes.push_back(rem);
            copy >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: Sign::Pos,
        };
    }
}

impl From<u32> for CASNum {
    fn from(value: u32) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255) as u8;
            bytes.push_back(rem);
            copy >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: Sign::Pos,
        };
    }
}

impl From<u64> for CASNum {
    fn from(value: u64) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255) as u8;
            bytes.push_back(rem);
            copy >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: Sign::Pos,
        };
    }
}

impl From<u128> for CASNum {
    fn from(value: u128) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255) as u8;
            bytes.push_back(rem);
            copy >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: Sign::Pos,
        };
    }
}

impl From<i8> for CASNum {
    fn from(value: i8) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();

        if value != 0 {
            bytes.push_back(value as u8);
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 },
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i16> for CASNum {
    fn from(value: i16) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs = value.abs();
        while abs > 0 {
            let rem: u8 = (abs & 255) as u8;
            bytes.push_back(rem);
            abs >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i32> for CASNum {
    fn from(value: i32) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs = value.abs();
        while abs > 0 {
            let rem: u8 = (abs & 255) as u8;
            bytes.push_back(rem);
            abs >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i64> for CASNum {
    fn from(value: i64) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs = value.abs();
        while abs > 0 {
            let rem: u8 = (abs & 255) as u8;
            bytes.push_back(rem);
            abs >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
            sign: if value > 0 { Sign::Pos } else { Sign::Neg },
        };
    }
}

impl From<i128> for CASNum {
    fn from(value: i128) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs = value.abs();
        while abs > 0 {
            let rem: u8 = (abs & 255) as u8;
            bytes.push_back(rem);
            abs >>= 8;
        }
        while let Some(&last) = bytes.back() {
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite { bytes, exp: 0 }.normalize(),
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
                        bytes: VecDeque::new(),
                        exp: 0,
                    },
                    sign: Sign::Pos,
                };
            }
            _ => {}
        }
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let bits = value.to_bits();
        const SIGN_MASK: u32 = 0x80000000;
        const MANTISSA_MASK: u32 = 0x007FFFFF;
        let sign: Sign = if bits & SIGN_MASK == 0 {
            Sign::Pos
        } else {
            Sign::Neg
        };
        let exp: i64 = i64::from((bits >> 23) & 255) - 150;
        let mut mantissa: u64 = u64::from(bits & MANTISSA_MASK) + 0x00800000;
        //fp values are 1.(mantissa) * 2^exp * (-1)^sign
        //so we add the 1 back in

        if exp > 0 {
            //we have to change mantissa since we cant have exponents that arent powers of 256
            mantissa <<= exp % 8;
        } else {
            mantissa >>= (-exp) % 8;
        }

        while mantissa > 0 {
            bytes.push_back((mantissa & 255).try_into().unwrap());
            mantissa /= 256;
            if bytes.len() == 3 {
                break;
            }
        }

        return CASNum {
            value: CASValue::Finite {
                bytes,
                exp: i128::from(exp / 8),
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
//             return match self.bytes.get((-min_digit).try_into().unwrap()) {
//                 Some(x) => *x,
//                 None => 0,
//             };
//         }
//     }
// }

impl Into<u16> for CASNum {
    fn into(self) -> u16 {
        todo!()
    }
}

impl Into<u32> for CASNum {
    fn into(self) -> u32 {
        todo!()
    }
}

impl Into<u64> for CASNum {
    fn into(self) -> u64 {
        todo!()
    }
}

impl Into<u128> for CASNum {
    fn into(self) -> u128 {
        todo!()
    }
}

impl Into<i8> for CASNum {
    fn into(self) -> i8 {
        todo!()
    }
}

impl Into<i16> for CASNum {
    fn into(self) -> i16 {
        todo!()
    }
}

impl Into<i32> for CASNum {
    fn into(self) -> i32 {
        todo!()
    }
}

impl Into<i64> for CASNum {
    fn into(self) -> i64 {
        todo!()
    }
}

impl Into<i128> for CASNum {
    fn into(self) -> i128 {
        todo!()
    }
}

impl Into<f32> for CASNum {
    fn into(self) -> f32 {
        todo!()
    }
}

impl Into<f64> for CASNum {
    fn into(self) -> f64 {
        todo!()
    }
}
