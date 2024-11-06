//for converting primitives to CASNum and vice versa

use std::collections::VecDeque;

use super::{CASNum, Sign};

pub trait CASNumConvert {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized;

    fn from_cas_num(casnum: CASNum) -> Self;
}

impl CASNumConvert for u8 {
    fn to_cas_num(&self) -> CASNum {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = *self;
        while copy > 0 {
            let rem: u8 = copy & 255;
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

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for u16 {
    fn to_cas_num(&self) -> CASNum {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = *self;
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

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for u32 {
    fn to_cas_num(&self) -> CASNum {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = *self;
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

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for u64 {
    fn to_cas_num(&self) -> CASNum {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = *self;
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

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for u128 {
    fn to_cas_num(&self) -> CASNum {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut copy = *self;
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

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for i8 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u8 = self.abs().try_into().unwrap();
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
            sign: if *self >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for i16 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u8 = self.abs().try_into().unwrap();
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
            sign: if *self >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for i32 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u8 = self.abs().try_into().unwrap();
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
            sign: if *self >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for i64 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u8 = self.abs().try_into().unwrap();
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
            sign: if *self >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for i128 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs: u8 = self.abs().try_into().unwrap();
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
            sign: if *self >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return out;
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for f32 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}

impl CASNumConvert for f64 {
    fn to_cas_num(&self) -> CASNum
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_cas_num(casnum: CASNum) -> Self {
        todo!()
    }
}
