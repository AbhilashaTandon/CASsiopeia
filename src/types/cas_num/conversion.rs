//for converting primitives to CASNum and vice versa

use std::collections::VecDeque;

use super::{CASNum, Sign};

impl From<u8> for CASNum {
    fn from(value: u8) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy: u16 = value.into();
        while copy > 0 {
            let rem: u8 = (copy & 255).try_into().unwrap();
            bytes.push_back(rem);
            copy /= 256;
        }
        while let Some(&last) = bytes.back() {
            //get rid of leading 0s
            if last == 0 {
                bytes.pop_back();
            } else {
                break;
            }
        }

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
        out.normalize();
        return out;
    }
}

impl From<u16> for CASNum {
    fn from(value: u16) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
        out.normalize();
        return out;
    }
}

impl From<u32> for CASNum {
    fn from(value: u32) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
        out.normalize();
        return out;
    }
}

impl From<u64> for CASNum {
    fn from(value: u64) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
        out.normalize();
        return out;
    }
}

impl From<u128> for CASNum {
    fn from(value: u128) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = value;
        while copy > 0 {
            let rem: u8 = (copy & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
        out.normalize();
        return out;
    }
}

impl From<i8> for CASNum {
    fn from(value: i8) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u16 = value.abs().try_into().unwrap();
        while abs > 0 {
            let rem: u8 = (abs & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: if value >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }
}

impl From<i16> for CASNum {
    fn from(value: i16) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u32 = value.abs().try_into().unwrap();
        while abs > 0 {
            let rem: u8 = (abs & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: if value >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }
}

impl From<i32> for CASNum {
    fn from(value: i32) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u64 = value.abs().try_into().unwrap();
        while abs > 0 {
            let rem: u8 = (abs & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: if value >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }
}

impl From<i64> for CASNum {
    fn from(value: i64) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u128 = value.abs().try_into().unwrap();
        while abs > 0 {
            let rem: u8 = (abs & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: if value >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }
}

impl From<i128> for CASNum {
    fn from(value: i128) -> Self {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u128 = value.abs().try_into().unwrap();
        while abs > 0 {
            let rem: u8 = (abs & 255).try_into().unwrap();
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

        let mut out = CASNum {
            bytes,
            exp: 0,
            sign: if value >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }
}

impl From<f32> for CASNum {
    fn from(value: f32) -> CASNum
    where
        Self: Sized,
    {
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
        println!("{}", exp);
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
        }

        return CASNum {
            bytes,
            exp: i128::from(exp / 8),
            sign,
        };
    }
}
