// use std::cmp::max;
// use std::ops;
// use std::path::Iter;

use std::{cmp::max, collections::VecDeque};

use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::{min, Ordering};

mod comp;
mod conversion;
mod iter;
mod operators;
mod test;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Sign {
    Pos,
    Neg,
}

#[derive(Debug)]
pub(crate) struct CASNum {
    pub(crate) value: CASValue,
    pub(crate) sign: Sign,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CASValue {
    Finite {
        bytes: VecDeque<u8>, //little endian
        exp: i128,           //base 256
    },
    Infinite,
    Indeterminate,
}

const INFINITY: CASNum = CASNum {
    value: CASValue::Infinite,
    sign: Sign::Pos,
};

const NEG_INFINITY: CASNum = CASNum {
    value: CASValue::Infinite,
    sign: Sign::Neg,
};

const INDETERMINATE: CASNum = CASNum {
    value: CASValue::Indeterminate,
    sign: Sign::Pos,
};

const ZERO: CASNum = CASNum {
    value: CASValue::Finite {
        bytes: VecDeque::new(),
        exp: 0,
    },
    sign: Sign::Pos,
};

impl CASNum {
    //only put functions in here instead of CASValue if they interact with sign
    pub(crate) fn abs(self) -> Self {
        return CASNum {
            sign: Sign::Pos,
            value: self.value,
        };
    }

    fn compare_finite(&self, other: &CASNum) -> Ordering {
        match (self.value.is_zero(), other.value.is_zero()) {
            (true, true) => return Equal, //0 == 0
            (true, false) => {
                if other.sign == Sign::Pos {
                    return Less; //0 < x
                } else {
                    return Greater; //0 < -x
                }
            }
            (false, true) => {
                if self.sign == Sign::Pos {
                    return Greater; //x > 0
                } else {
                    return Less; //-x < 0
                }
            }
            (false, false) => {}
        };
        match (self, other) {
            (
                CASNum {
                    value: CASValue::Finite { .. },
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Finite { .. },
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (Sign::Pos, Sign::Pos) => {
                    let self_max_digit = self.value.max_digit().unwrap();
                    let other_max_digit = other.value.max_digit().unwrap();

                    println!("{} {}", self_max_digit, other_max_digit);
                    //we can safely unwrap since these are finite

                    if self_max_digit > other_max_digit {
                        return Greater;
                    } else if self_max_digit < other_max_digit {
                        return Less;
                    } else {
                        let alignment = self.value.align(&other.value).unwrap();
                        for (a_byte, b_byte, _) in alignment.iter().rev() {
                            match a_byte.cmp(b_byte) {
                                Less => return Less,
                                Equal => continue,
                                Greater => return Greater,
                            }
                        }
                        return Equal;
                    }
                }
                (Sign::Pos, Sign::Neg) => Greater,
                (Sign::Neg, Sign::Pos) => Less,
                (Sign::Neg, Sign::Neg) => {
                    {
                        let self_max_digit = self.value.max_digit().unwrap();
                        let other_max_digit = other.value.max_digit().unwrap();

                        //we can safely unwrap since these are finite

                        if self_max_digit > other_max_digit {
                            return Less;
                        } else if self_max_digit < other_max_digit {
                            return Greater;
                        } else {
                            let alignment = self.value.align(&other.value).unwrap();
                            for (a_byte, b_byte, _) in alignment.iter().rev() {
                                match a_byte.cmp(b_byte) {
                                    Less => return Greater,
                                    Equal => continue,
                                    Greater => return Less,
                                }
                            }
                            return Equal;
                        }
                    }
                }
            },
            _ => {
                assert!(false);
                return Equal;
            }
        }
    }
}

impl CASValue {
    pub(crate) fn new<T>(i: T) -> CASNum
    where
        CASNum: From<T>,
    {
        return i.into();
    }

    fn max_digit(self: &Self) -> Option<i128> {
        //exponent position of first digit
        return match self {
            CASValue::Finite { bytes, exp } => Some((bytes.len() as i128) - 1 + exp),
            CASValue::Infinite => None,
            CASValue::Indeterminate => None,
        };
    }

    pub(crate) fn normalize(mut self: Self) -> Self {
        match self {
            CASValue::Finite {
                ref mut bytes,
                ref mut exp,
            } => {
                while let Some(least_order_byte) = bytes.front() {
                    if *least_order_byte == 0 {
                        //if has trailing 0 remove it
                        *exp += 1;
                        bytes.pop_front();
                    } else {
                        break;
                    }
                }
                self
            }
            CASValue::Infinite => self,
            CASValue::Indeterminate => self,
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        match &self {
            CASValue::Finite { bytes, .. } => {
                for byte in bytes {
                    //this could be made more efficient
                    if *byte != 0 {
                        return false;
                    }
                }
                return true;
            }
            CASValue::Infinite => false,
            CASValue::Indeterminate => false,
        }
    }

    pub(crate) fn is_infinite(&self) -> bool {
        match &self {
            CASValue::Finite { .. } => false,
            CASValue::Infinite => true,
            CASValue::Indeterminate => false,
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        match &self {
            CASValue::Finite { .. } => true,
            CASValue::Infinite => false,
            CASValue::Indeterminate => false,
        }
    }

    pub(crate) fn is_indeterminate(&self) -> bool {
        match &self {
            CASValue::Finite { .. } => false,
            CASValue::Infinite => false,
            CASValue::Indeterminate => true,
        }
    }

    pub fn align(self: &Self, other: &Self) -> Option<VecDeque<(u8, u8, i128)>> {
        //digits aligned by exponent and zipped together
        //base 10 example

        //1200, .003
        // (1, 0, 3) (2, 0, 2) (0, 0, 1) (0, 0, 0) . (0, 0, -1) (0, 0, -2) (0, 3, -3)
        //thousands place, hundreds place, tens place, ones place, tenths place, hundredths place, thousandths place

        if let CASValue::Finite {
            bytes: self_bytes,
            exp: self_exp,
        } = self
        {
            if let CASValue::Finite {
                bytes: other_bytes,
                exp: other_exp,
            } = other
            {
                let a_max_digit = self.max_digit().unwrap(); //we can safely unwrap this since it only returns none if not finite
                let a_min_digit = *self_exp;
                let b_max_digit = other.max_digit().unwrap(); //we can safely unwrap this since it only returns none if not finite
                let b_min_digit = *other_exp;
                let max_digit = max(a_max_digit, b_max_digit);
                let min_digit = min(a_min_digit, b_min_digit);

                let mut out: VecDeque<(u8, u8, i128)> = VecDeque::new();
                for i in min_digit..=max_digit {
                    out.push_back((
                        if a_min_digit <= i && i <= a_max_digit {
                            self_bytes[(i - a_min_digit).try_into().unwrap()]
                        } else {
                            0
                        },
                        if b_min_digit <= i && i <= b_max_digit {
                            other_bytes[(i - b_min_digit).try_into().unwrap()]
                        } else {
                            0
                        },
                        i,
                    ));
                }
                return Some(out);
            }
        }

        return None;
    }

    pub fn cartesian(&self, other: &Self) -> Option<VecDeque<VecDeque<(u8, u8, i128)>>> {
        //aligned cartesian product of base 256 digits
        //base 10 example

        //123.45, 4.567 ->
        // (1, 4,  2) (2, 4,  1) (3, 4,  0) . (4, 4, -1) (5, 4, -2)
        //     .      .      .        .      .
        // (1, 5,  1) (2, 5,  0) (3, 5, -1) . (4, 5, -2) (5, 5, -3)
        // (1, 6,  0) (2, 6, -1) (3, 6, -2) . (4, 6, -3) (5, 6, -4)
        // (1, 7, -1) (2, 7, -2) (3, 7, -3) . (4, 7, -4) (5, 7, -5)

        if let CASValue::Finite {
            bytes: self_bytes,
            exp: self_exp,
        } = self
        {
            if let CASValue::Finite {
                bytes: other_bytes,
                exp: other_exp,
            } = other
            {
                let self_max_digit = self.max_digit().unwrap(); //we can safely unwrap this since it only returns none if not finite
                let self_min_digit = *self_exp;
                let other_max_digit = other.max_digit().unwrap(); //we can safely unwrap this since it only returns none if not finite
                let other_min_digit = *other_exp;

                let mut out: VecDeque<VecDeque<(u8, u8, i128)>> = VecDeque::new();
                for i in self_min_digit..=self_max_digit {
                    let self_byte = self_bytes[(i - self_min_digit).try_into().unwrap()];

                    if self_byte == 0 {
                        //0s don't contribute to multiplication
                        continue;
                    }

                    let mut row: VecDeque<(u8, u8, i128)> = VecDeque::new();

                    for j in other_min_digit..=other_max_digit {
                        let other_byte = other_bytes[(j - other_min_digit).try_into().unwrap()];

                        if other_byte == 0 {
                            continue;
                        }
                        row.push_back((self_byte, other_byte, i + j));
                    }
                    out.push_back(row);
                }
                return Some(out);
            }
        }

        return None;
    }
}
