//numerical operators, +, -, *, / etc
use std::{collections::VecDeque, ops};

use super::{helper, CASNum, Sign};

impl ops::Neg for CASNum {
    type Output = CASNum;

    fn neg(self) -> Self::Output {
        return CASNum {
            bytes: self.bytes,
            exp: self.exp,
            sign: if self.sign == Sign::Pos {
                Sign::Neg
            } else {
                Sign::Pos
            },
        };
    }
}

impl ops::Add<CASNum> for CASNum {
    type Output = CASNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut carry = 0;

        if rhs.sign == Sign::Neg && self.sign == Sign::Pos {
            //a + -b = a - b
            return self - rhs.abs();
        }

        if self.sign == Sign::Neg && rhs.sign == Sign::Pos {
            //-a + b = b - a
            return rhs - self.abs();
        }

        if self.sign == Sign::Neg && rhs.sign == Sign::Neg {
            //-a + -b = - (a + b)
            return -(self.abs() + rhs.abs());
        }

        for (a_byte, b_byte, _) in helper::align(&self, &rhs) {
            let sum: u16 = a_byte as u16 + b_byte as u16 + carry;
            if sum >= 256 {
                carry = sum / 256;
            }
            let new_byte: u8 = (sum % 256).try_into().unwrap();
            bytes.push_back(new_byte);
        }

        return CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
    }
}

impl ops::Sub<CASNum> for CASNum {
    type Output = Self;

    fn sub(self: CASNum, rhs: Self) -> Self::Output {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        //convert all args to positive
        if rhs.sign == Sign::Neg && self.sign == Sign::Pos {
            //a - -b = a + b
            return self + rhs.abs();
        }

        if self.sign == Sign::Neg && rhs.sign == Sign::Pos {
            //-a - b = a - b
            return -(rhs + self.abs());
        }

        if self.sign == Sign::Neg && rhs.sign == Sign::Neg {
            //-a - -b = -a + b = b - a
            return rhs.abs() - self.abs();
        }

        if self < rhs {
            //a - b = -(b - a)
            return -(rhs - self);
        }

        let mut carry: i16 = 0;
        for (self_byte, other_byte, _) in helper::align(&self, &rhs).iter() {
            let mut diff: i16 = (*self_byte as i16) - (*other_byte as i16) - carry;

            if diff < 0 {
                diff = 255 + diff;
                carry = 1;
            }
            bytes.push_back(diff.try_into().unwrap());
        }

        return CASNum {
            bytes,
            exp: 0,
            sign: Sign::Pos,
        };
    }
}
