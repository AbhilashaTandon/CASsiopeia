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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum Sign {
    Pos,
    Neg,
}

#[derive(Debug, Clone)]
pub(crate) struct CASNum {
    pub(crate) value: CASValue,
    pub(crate) sign: Sign,
}

#[derive(Debug, Clone)]
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

impl CASNum {
    //only put functions in here instead of CASValue if they interact with sign
    pub(crate) fn abs(self) -> Self {
        return CASNum {
            sign: Sign::Pos,
            value: self.value,
        };
    }

    fn compare_finite(
        &self,
        self_sign: &Sign,
        other_sign: &Sign,
        self_bytes: &VecDeque<u8>,
        other_bytes: &VecDeque<u8>,
        other: &CASNum,
    ) -> Option<Ordering> {
        match (self_sign, other_sign) {
            (Sign::Pos, Sign::Pos) => {
                match self_bytes.len().cmp(&other_bytes.len()) {
                    Less => return Some(Less),
                    Greater => return Some(Greater),
                    Equal => {}
                };
                let alignment = self.value.align(&other.value);
                if alignment.is_none() {
                    return None;
                }
                for (a_byte, b_byte, _) in alignment.unwrap().iter().rev() {
                    match a_byte.cmp(b_byte) {
                        Less => return Some(Less),
                        Equal => continue,
                        Greater => return Some(Greater),
                    }
                }
                return Some(Equal);
            }
            (Sign::Pos, Sign::Neg) => Some(Greater),
            (Sign::Neg, Sign::Pos) => Some(Less),
            (Sign::Neg, Sign::Neg) => match other.partial_cmp(self) {
                // -b < -a == a < b
                Some(Greater) => Some(Less),
                Some(Less) => Some(Greater),
                Some(Equal) => Some(Equal),
                None => None,
            },
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
                exp: mut _exp,
            } => {
                while let Some(least_order_byte) = bytes.front() {
                    if *least_order_byte == 0 {
                        //if has trailing 0 remove it
                        _exp += 1;
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
                        false;
                    }
                }
                true
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
                    let mut row: VecDeque<(u8, u8, i128)> = VecDeque::new();
                    for j in other_min_digit..=other_max_digit {
                        row.push_back((
                            if self_min_digit <= i && i <= self_max_digit {
                                self_bytes[(i - self_min_digit).try_into().unwrap()]
                            } else {
                                0
                            },
                            if other_min_digit <= j && j <= other_max_digit {
                                other_bytes[(j - other_min_digit).try_into().unwrap()]
                            } else {
                                0
                            },
                            i + j,
                        ));
                    }
                    out.push_back(row);
                }
                return Some(out);
            }
        }

        return None;
    }
}
