// use std::cmp::max;
// use std::ops;
// use std::path::Iter;

use std::{
    cmp::{max, min, Ordering},
    collections::VecDeque,
    ops,
};

mod test;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Sign {
    Pos,
    Neg,
}

#[derive(Debug)]
pub(crate) struct CASNum {
    pub(crate) bytes: VecDeque<u8>, //little endian
    pub(crate) exp: i128,           //base 256
    pub(crate) sign: Sign,
}

pub(crate) struct CASNumIter {
    cas_num: CASNum,
    index: usize,
}

impl Iterator for CASNumIter {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
        let current = self.cas_num.bytes.get(self.index);

        match current {
            Some(current) => {
                self.index += 1;
                Some(match self.cas_num.sign {
                    Sign::Pos => *current as i16,
                    Sign::Neg => -(*current as i16),
                })
            }
            None => None,
        }
    }
}

impl IntoIterator for CASNum {
    type Item = i16;
    type IntoIter = CASNumIter;

    fn into_iter(self) -> Self::IntoIter {
        CASNumIter {
            cas_num: self,
            index: 0,
        }
    }
}

pub(crate) fn align(a: &CASNum, b: &CASNum) -> VecDeque<(u8, u8)> {
    let a_max_digit = a.max_digit();
    let a_min_digit = a.exp;
    let b_max_digit = b.max_digit();
    let b_min_digit = b.exp;
    let max_digit = max(a_max_digit, b_max_digit);
    let min_digit = min(a_min_digit, b_min_digit);

    let mut out: VecDeque<(u8, u8)> = VecDeque::new();
    for i in min_digit..=max_digit {
        out.push_back((
            if a_min_digit <= i && i <= a_max_digit {
                a.bytes[(i - a_min_digit).try_into().unwrap()]
            } else {
                0
            },
            if b_min_digit <= i && i <= b_max_digit {
                b.bytes[(i - b_min_digit).try_into().unwrap()]
            } else {
                0
            },
        ));
    }

    return out;
}

impl PartialEq<CASNum> for CASNum {
    fn eq(&self, other: &CASNum) -> bool {
        self.bytes == other.bytes && self.exp == other.exp && self.sign == other.sign
    }
}

fn compare(a: &CASNum, b: &CASNum) -> Ordering {
    //returns comparison of absolute values
    match a.bytes.len().cmp(&b.bytes.len()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {}
    }
    for (a_byte, b_byte) in align(a, b).iter().rev() {
        match a_byte.cmp(b_byte) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
        }
    }

    return Ordering::Equal;
}

impl Eq for CASNum {}
impl PartialOrd for CASNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sign == Sign::Pos && other.sign == Sign::Pos {
            return Some(compare(self, other));
        } else if self.sign == Sign::Neg && other.sign == Sign::Pos {
            return Some(Ordering::Less);
        } else if self.sign == Sign::Pos && other.sign == Sign::Neg {
            return Some(Ordering::Greater);
        } else {
            return Some(compare(other, self));
        }
    }
}

impl CASNum {
    pub(crate) fn new(i: i128) -> Box<CASNum> {
        let mut bytes: VecDeque<u8> = VecDeque::new();
        let mut abs = i.abs();
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
            sign: if i >= 0 { Sign::Pos } else { Sign::Neg },
        };
        out.normalize();
        return Box::new(out);
    }

    fn max_digit(self: &Self) -> i128 {
        //exponent position of first digit
        return (self.bytes.len() as i128) - 1 + self.exp;
    }

    pub(crate) fn normalize(self: &mut Self) {
        while let Some(least_order_byte) = self.bytes.front() {
            if *least_order_byte == 0 {
                //if has trailing 0 remove it
                self.exp += 1;
                self.bytes.pop_front();
            } else {
                break;
            }
        }
    }

    pub(crate) fn abs(self) -> CASNum {
        return CASNum {
            bytes: self.bytes,
            exp: self.exp,
            sign: Sign::Pos,
        };
    }
}

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
            return self - rhs.abs();
        }

        if self.sign == Sign::Neg && rhs.sign == Sign::Pos {
            return rhs - self.abs();
        }

        if self.sign == Sign::Neg && rhs.sign == Sign::Neg {
            return -(self.abs() + rhs.abs());
        }

        for (a_byte, b_byte) in align(&self, &rhs) {
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
        for (self_byte, other_byte) in align(&self, &rhs).iter() {
            let mut diff: i16 = (*self_byte as i16) - (*other_byte as i16) - carry;
            if diff < 0 {
                diff = 256 - diff;
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
