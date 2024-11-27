use std::{cmp::max, collections::VecDeque};

use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::{min, Ordering};

use std::fmt::Debug;

type DigitType = u64;
const NUM_BITS: i128 = 64;

mod comp;
mod conversion;
mod iter;
mod literal;
mod operators;
mod test;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
enum Sign {
    Pos,
    Neg,
}

#[derive(Clone, Hash, Eq)]
pub(crate) struct CASNum {
    pub(crate) value: CASValue,
    pub(self) sign: Sign,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub(crate) enum CASValue {
    Finite {
        digits: VecDeque<DigitType>, //little endian
        exp: isize,                  //base 256
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

pub(crate) const ZERO: CASNum = CASNum {
    value: CASValue::Finite {
        digits: VecDeque::new(),
        exp: 0,
    },
    sign: Sign::Pos,
};

impl CASNum {
    fn new<T>(i: T) -> CASNum
    where
        CASNum: From<T>,
    {
        i.into()
    }
    //only put functions in here instead of CASValue if they interact with sign
    fn abs(&self) -> Self {
        CASNum {
            sign: Sign::Pos,
            value: self.value.clone(),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
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
                    value: CASValue::Finite { exp: self_exp, .. },
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Finite { exp: other_exp, .. },
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (Sign::Pos, Sign::Pos) => {
                    let self_max_digit = self_exp;
                    let other_max_digit = other_exp;
                    //we can safely unwrap since these are finite

                    match self_max_digit.cmp(other_max_digit) {
                        Greater => Greater,
                        Less => Less,
                        Equal => {
                            let alignment = self.value.align(&other.value).unwrap();
                            for (a_num, b_num, _) in alignment.iter().rev() {
                                match a_num.cmp(b_num) {
                                    Less => return Less,
                                    Equal => continue,
                                    Greater => return Greater,
                                }
                            }
                            Equal
                        }
                    }
                }
                (Sign::Pos, Sign::Neg) => Greater,
                (Sign::Neg, Sign::Pos) => Less,
                (Sign::Neg, Sign::Neg) => {
                    {
                        let self_max_digit = self_exp;
                        let other_max_digit = other_exp;

                        //we can safely unwrap since these are finite

                        match self_max_digit.cmp(other_max_digit) {
                            Less => Greater,
                            Greater => Less,
                            Equal => {
                                let alignment = self.value.align(&other.value).unwrap();
                                for (a_num, b_num, _) in alignment.iter().rev() {
                                    match a_num.cmp(b_num) {
                                        Less => return Greater,
                                        Equal => continue,
                                        Greater => return Less,
                                    }
                                }
                                Equal
                            }
                        }
                    }
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
}

impl CASValue {
    fn exp(&self) -> Option<isize> {
        match self {
            CASValue::Finite { exp, .. } => Some(*exp),
            CASValue::Infinite => None,
            CASValue::Indeterminate => None,
        }
    }

    fn normalize(mut self) -> Self {
        match self {
            CASValue::Finite {
                ref mut digits,
                ref mut exp,
            } => {
                while let Some(least_order_num) = digits.front() {
                    if *least_order_num == 0 {
                        //if has trailing 0 remove it
                        digits.pop_front();
                    } else {
                        break;
                    }
                }
                while let Some(least_order_num) = digits.back() {
                    if *least_order_num == 0 {
                        //if has leading 0 remove it
                        digits.pop_back();
                        *exp -= 1;
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

    fn is_zero(&self) -> bool {
        match &self {
            CASValue::Finite { digits, .. } => {
                for num in digits {
                    //this could be made more efficient
                    if *num != 0 {
                        return false;
                    }
                }
                true
            }
            CASValue::Infinite => false,
            CASValue::Indeterminate => false,
        }
    }

    fn is_infinite(&self) -> bool {
        match &self {
            CASValue::Finite { .. } => false,
            CASValue::Infinite => true,
            CASValue::Indeterminate => false,
        }
    }

    fn is_finite(&self) -> bool {
        match &self {
            CASValue::Finite { .. } => true,
            CASValue::Infinite => false,
            CASValue::Indeterminate => false,
        }
    }

    fn is_indeterminate(&self) -> bool {
        match &self {
            CASValue::Finite { .. } => false,
            CASValue::Infinite => false,
            CASValue::Indeterminate => true,
        }
    }

    fn align(&self, other: &Self) -> Option<VecDeque<(DigitType, DigitType, isize)>> {
        //digits aligned by exponent and zipped together
        //base 10 example

        //1200, .003
        // (1, 0, 3) (2, 0, 2) (0, 0, 1) (0, 0, 0) . (0, 0, -1) (0, 0, -2) (0, 3, -3)
        //thousands place, hundreds place, tens place, ones place, tenths place, hundredths place, thousandths place

        let self_digits: &VecDeque<DigitType>;
        let self_exp: isize;

        let other_digits: &VecDeque<DigitType>;
        let other_exp: isize;

        match self {
            CASValue::Finite { digits, exp } => {
                self_digits = digits;
                self_exp = *exp;
            }
            _ => return None,
        }

        match other {
            CASValue::Finite { digits, exp } => {
                other_digits = digits;
                other_exp = *exp;
            }
            _ => return None,
        }

        let self_max_exp: isize = self_exp;
        //exponent of max digit of self_digits
        let self_min_exp = self_exp - (self_digits.len() - 1) as isize;
        //exponent of min digit of self_digits

        let other_max_exp = other_exp;
        //exponent of max digit of other_digits
        let other_min_exp = other_exp - (other_digits.len() - 1) as isize;

        //exponent of min digit of other_digits

        let max_exp = max(self_max_exp, other_max_exp);
        let min_exp = min(self_min_exp, other_min_exp);

        let mut out: VecDeque<(DigitType, DigitType, isize)> = VecDeque::new();
        for i in min_exp..=max_exp {
            out.push_back((
                if self_min_exp <= i && i <= self_max_exp {
                    self_digits[(i - self_min_exp).try_into().unwrap()]
                } else {
                    0
                },
                if other_min_exp <= i && i <= other_max_exp {
                    other_digits[(i - other_min_exp).try_into().unwrap()]
                } else {
                    0
                },
                i,
            ));
        }
        Some(out)
    }

    fn cartesian(&self, other: &Self) -> Option<VecDeque<VecDeque<(DigitType, DigitType, isize)>>> {
        //aligned cartesian product of base 256 digits
        //base 10 example

        //120.45, 4.067 ->
        // (1, 4,  2) (2, 4,  1) . (4, 4, -1) (5, 4, -2)
        //     .      .      .        .      .
        // (1, 6,  0) (2, 6, -1)  . (4, 6, -3) (5, 6, -4)
        // (1, 7, -1) (2, 7, -2)  . (4, 7, -4) (5, 7, -5)

        if let CASValue::Finite {
            digits: self_digits,
            exp: self_exp,
        } = self
        {
            if let CASValue::Finite {
                digits: other_digits,
                exp: other_exp,
            } = other
            {
                let self_max_exp: isize = *self_exp;
                //exponent of max digit of self_digits
                let self_min_exp = self_max_exp - (self_digits.len() - 1) as isize;
                //exponent of min digit of self_digits

                let other_max_exp = *other_exp;
                //exponent of max digit of other_digits
                let other_min_exp = other_max_exp - (other_digits.len() - 1) as isize;

                let mut out: VecDeque<VecDeque<(DigitType, DigitType, isize)>> = VecDeque::new();
                for i in self_min_exp..=self_max_exp {
                    let self_num = self_digits[(i - self_min_exp).try_into().unwrap()];

                    if self_num == 0 {
                        //0s don't contribute to multiplication
                        continue;
                    }

                    let mut row: VecDeque<(DigitType, DigitType, isize)> = VecDeque::new();

                    for j in other_min_exp..=other_max_exp {
                        let other_num = other_digits[(j - other_min_exp).try_into().unwrap()];

                        if other_num == 0 {
                            continue;
                        }
                        row.push_back((self_num, other_num, i + j));
                    }
                    out.push_back(row);
                }
                return Some(out);
            }
        }

        None
    }

    fn set_precision(&mut self, num_digits: usize) {
        if let CASValue::Finite { digits, exp } = self {
            while digits.len() > num_digits {
                digits.pop_front();
                *exp += 1;
            }
        }
    }
}

use std::fmt::Display;

impl Display for CASNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let float: f64 = <CASNum as Clone>::clone(self).into();
        //TODO: get rid of this clone
        match self {
            CASNum {
                value: CASValue::Finite { digits, exp },
                sign,
            } => {
                let mut hex_str: String = String::new();
                for digit in digits.iter().rev() {
                    hex_str += &format!("{:0>2x}", digit);
                }
                write!(
                    f,
                    "0x{}{} x (2^64) ^ {} ({:e})",
                    if *sign == Sign::Pos { "" } else { "-" },
                    hex_str,
                    exp,
                    float,
                )
            }
            CASNum {
                value: CASValue::Infinite,
                sign: Sign::Pos,
            } => write!(f, "∞"),
            CASNum {
                value: CASValue::Infinite,
                sign: Sign::Neg,
            } => write!(f, "-∞"),
            CASNum {
                value: CASValue::Indeterminate,
                ..
            } => write!(f, "NaN"),
        }
    }
}

impl Debug for CASNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CASNum {
                value: CASValue::Infinite { .. },
                sign,
            } => {
                if *sign == Sign::Pos {
                    write!(f, "∞")
                } else {
                    write!(f, "-∞")
                }
            }

            CASNum {
                value: CASValue::Indeterminate,
                ..
            } => write!(f, "NaN"),

            CASNum {
                value: CASValue::Finite { digits, exp },
                sign,
            } => write!(
                f,
                "{}{:?}e{}",
                if *sign == Sign::Pos { "" } else { "-" },
                digits,
                exp
            ),
        }
    }
}
