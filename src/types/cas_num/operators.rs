//numerical operators, +, -, *, / etc
use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use num_traits::Zero;

use crate::types::cas_num::{DigitType, NUM_BITS};

use super::{CASNum, CASValue, Sign, INDETERMINATE, INFINITY, NEG_INFINITY, ZERO};

impl Neg for CASNum {
    type Output = CASNum;
    fn neg(self) -> Self::Output {
        CASNum {
            value: self.value,
            sign: if self.sign == Sign::Pos {
                Sign::Neg
            } else {
                Sign::Pos
            },
        }
    }
}

impl<'a, 'b> AddAssign<&'b CASNum> for &'a CASNum
where
    'b: 'a,
{
    fn add_assign(&mut self, rhs: &'b CASNum) {
        *self = match (&self, &rhs) {
            (
                CASNum {
                    value: CASValue::Finite { .. },
                    ..
                },
                CASNum {
                    value: CASValue::Finite { .. },
                    ..
                },
            ) => {
                match (self.is_zero(), rhs.is_zero()) {
                    (true, true) => CASNum::from(0), //0 + 0 == 0
                    (true, false) => rhs.clone(),    //0 + x == x
                    (false, true) => return,         //x + 0 == x
                    (false, false) => addition_finite(&self, &rhs),
                }
            }
            (
                CASNum {
                    value: CASValue::Finite { .. },
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Infinite,
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (_, Sign::Pos) => INFINITY,     //finite + inf == inf
                (_, Sign::Neg) => NEG_INFINITY, //finite + -inf == -inf
            },
            (
                CASNum {
                    value: CASValue::Infinite,
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Finite { .. },
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (Sign::Pos, _) => INFINITY,     //inf + finite = inf
                (Sign::Neg, _) => NEG_INFINITY, //-inf + finite = -inf
            },
            (
                CASNum {
                    value: CASValue::Infinite,
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Infinite,
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (Sign::Pos, Sign::Pos) => INFINITY,      //iinf + inf == inf
                (Sign::Pos, Sign::Neg) => INDETERMINATE, //inf + -inf == nan
                (Sign::Neg, Sign::Pos) => INDETERMINATE, //-inf + inf == nan
                (Sign::Neg, Sign::Neg) => NEG_INFINITY,  //-inf + -inf == -inf
            },
            _ => INDETERMINATE, //indeterminate (NaN) + anyting is nan
        };
    }
}

fn addition_finite(lhs: &CASNum, rhs: &CASNum) -> CASNum {
    match (lhs.sign, rhs.sign) {
        (Sign::Pos, Sign::Pos) => {
            let mut digits: VecDeque<DigitType> = VecDeque::new();
            let mut carry = 0;

            let alignment = &lhs.value.align(&rhs.value).unwrap(); //we can unwrap safely since both self and rhs are finite

            let exp = alignment.back().unwrap().2;

            for (a_digit, b_digit, _) in alignment {
                let mut sum: u128 = *a_digit as u128 + *b_digit as u128 + carry;
                if sum > 0xFFFFFFFFFFFFFFFF {
                    //if carry
                    carry = 1;
                    sum -= 0x10000000000000000;
                } else {
                    carry = 0;
                }
                let new_digit: DigitType = sum as u64;
                digits.push_back(new_digit);
            }

            if carry != 0 {
                digits.push_back(carry as DigitType);
            }

            CASNum {
                value: CASValue::Finite { digits, exp },
                sign: Sign::Pos,
            }
        }

        (Sign::Pos, Sign::Neg) => subtraction_finite(lhs, &rhs.abs()),
        (Sign::Neg, Sign::Pos) => subtraction_finite(rhs, &lhs.abs()),
        (Sign::Neg, Sign::Neg) => -addition_finite(&lhs.abs(), &rhs.abs()),
    }
}

impl SubAssign<CASNum> for CASNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = match (&self, &rhs) {
            (
                CASNum {
                    value: CASValue::Finite { .. },
                    ..
                },
                CASNum {
                    value: CASValue::Finite { .. },
                    ..
                },
            ) => {
                match (self.value.is_zero(), rhs.value.is_zero()) {
                    (true, true) => CASNum::from(0), //0 - 0 == 0
                    (true, false) => -rhs,           //0 - x == -x
                    (false, true) => return,         //x - 0 == x
                    (false, false) => subtraction_finite(self, &rhs),
                }
            }
            (
                CASNum {
                    value: CASValue::Finite { .. },
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Infinite,
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (_, Sign::Pos) => NEG_INFINITY, //finite - inf == -inf
                (_, Sign::Neg) => INFINITY,     //finite - -inf == inf
            },
            (
                CASNum {
                    value: CASValue::Infinite,
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Finite { .. },
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (Sign::Pos, _) => INFINITY,     //inf - finite = inf
                (Sign::Neg, _) => NEG_INFINITY, //-inf - finite = -inf
            },
            (
                CASNum {
                    value: CASValue::Infinite,
                    sign: self_sign,
                },
                CASNum {
                    value: CASValue::Infinite,
                    sign: other_sign,
                },
            ) => match (self_sign, other_sign) {
                (Sign::Pos, Sign::Pos) => INDETERMINATE, //inf - inf == nan
                (Sign::Pos, Sign::Neg) => INFINITY,      //inf - -inf == inf
                (Sign::Neg, Sign::Pos) => NEG_INFINITY,  //-inf - inf == -inf
                (Sign::Neg, Sign::Neg) => INDETERMINATE, //-inf - -inf == nan
            },
            _ => INDETERMINATE, //indeterminate (NaN) + anyting is nan
        }
    }
}

fn subtraction_finite(lhs: &CASNum, rhs: &CASNum) -> CASNum {
    match (lhs.sign, rhs.sign) {
        (Sign::Pos, Sign::Pos) => {
            if lhs < rhs {
                //a - b = -(b - a)
                return -subtraction_finite(rhs, lhs);
            }
            let mut digits: VecDeque<DigitType> = VecDeque::new();
            let mut carry = 0;

            let alignment = &lhs.value.align(&rhs.value).unwrap(); //we can unwrap safely since both lhs and rhs are finite

            let exp = alignment.back().unwrap().2;

            for (self_digit, other_digit, _) in alignment {
                let mut diff: i128 = (*self_digit as i128) - (*other_digit as i128) + carry;

                if diff < 0 {
                    diff += 0x10000000000000000;
                    carry = -1;
                } else {
                    carry = 0;
                }
                digits.push_back(diff.try_into().unwrap());
            }

            CASNum {
                value: CASValue::Finite { digits, exp },
                sign: Sign::Pos,
            }
        }

        (Sign::Pos, Sign::Neg) => addition_finite(lhs, &rhs.abs()), //a - -b = a + b
        (Sign::Neg, Sign::Pos) => -addition_finite(rhs, &lhs.abs()), ////-a - b = a + b
        (Sign::Neg, Sign::Neg) => subtraction_finite(&rhs.abs(), &lhs.abs()), //-a - -b = -a + b = b - a
    }
}

impl MulAssign<CASNum> for CASNum {
    fn mul_assign(&mut self, rhs: Self) {
        if self.value.is_indeterminate() || rhs.value.is_indeterminate() {
            *self = INDETERMINATE;
            return;
            //NAN * x == NAN
            //x * NAN == NAN
        }

        if self.value.is_zero() && rhs.value.is_infinite() {
            *self = INDETERMINATE;
            return;

            //0 * +/- inf == NAN
        }

        if self.value.is_infinite() && rhs.value.is_zero() {
            *self = INDETERMINATE;
            return;

            // +/- inf  * 0 == NAN
        }

        if self.value.is_infinite() || rhs.value.is_infinite() {
            *self = match (self.sign, rhs.sign) {
                (Sign::Pos, Sign::Pos) => INFINITY,
                (Sign::Pos, Sign::Neg) => NEG_INFINITY,
                (Sign::Neg, Sign::Pos) => NEG_INFINITY,
                (Sign::Neg, Sign::Neg) => INFINITY,
            };
            return;
        }

        if self.value.is_zero() || rhs.value.is_zero() {
            *self = ZERO;
            return;
        }

        *self = multiplication_finite(self, &rhs);
    }
}

fn multiplication_finite(lhs: &CASNum, rhs: &CASNum) -> CASNum {
    match (&lhs.sign, &rhs.sign) {
        (Sign::Pos, Sign::Pos) => {}
        (Sign::Pos, Sign::Neg) => return -multiplication_finite(lhs, &rhs.abs()),
        //a * -b = - (a * b)
        (Sign::Neg, Sign::Pos) => return -multiplication_finite(&lhs.abs(), rhs),
        //-a * b = -(a * b)
        (Sign::Neg, Sign::Neg) => return multiplication_finite(&lhs.abs(), &rhs.abs()),
        //-a * -b = a * b
    };

    let bit_mask: u128 = u64::MAX.into(); //all ones for extracting lower 64 bits

    let cartesian = lhs.value.cartesian(&rhs.value).unwrap();

    let max_digit = cartesian.back().unwrap().back().unwrap().2;
    let min_digit = cartesian.front().unwrap().front().unwrap().2;

    assert!(max_digit >= min_digit);

    let mut temp_arr: Vec<u128> = vec![0; (max_digit - min_digit + 1) as usize];

    for row in cartesian {
        for (self_digit, rhs_digit, exp) in row {
            temp_arr[(exp - min_digit) as usize] += (self_digit as u128) * (rhs_digit as u128);
        }
    }

    let mut carry = 0;
    let mut digits: VecDeque<DigitType> = VecDeque::new();

    for &value in temp_arr.iter() {
        let sum = value.checked_add(carry);
        assert!(sum.is_some());

        let adjusted = sum.unwrap() & bit_mask;
        carry = sum.unwrap() >> NUM_BITS;
        digits.push_back(adjusted as DigitType);
    }

    while carry > 0 {
        digits.push_back((carry & bit_mask) as u64);
        carry >>= NUM_BITS;
    }

    let exp = (digits.len() - temp_arr.len()) as isize
        + lhs.value.exp().unwrap()
        + rhs.value.exp().unwrap();
    let val = CASValue::Finite { digits, exp };

    CASNum {
        value: val.normalize(),
        sign: Sign::Pos,
    }
}

impl DivAssign<CASNum> for CASNum {
    fn div_assign(&mut self, rhs: Self) {
        if self.value.is_zero() && rhs.value.is_zero() {
            *self = INDETERMINATE;
            return;
            //0/0 == NAN
        }

        if self.value.is_zero() || rhs.value.is_infinite() {
            *self = ZERO;
            return;
        }

        if self.value.is_indeterminate() || rhs.value.is_indeterminate() {
            *self = INDETERMINATE;
            return;
            //NAN / x == NAN
            //x / NAN == NAN
        }

        if self.value.is_infinite() && rhs.value.is_infinite() {
            *self = INDETERMINATE;
            //inf / inf == nan
            return;
        }

        if self.value.is_infinite() {
            *self = match (self.sign, rhs.sign) {
                (Sign::Pos, Sign::Pos) => INFINITY,
                (Sign::Pos, Sign::Neg) => NEG_INFINITY,
                (Sign::Neg, Sign::Pos) => NEG_INFINITY,
                (Sign::Neg, Sign::Neg) => INFINITY,
            };
            return;
        }

        if let (
            CASNum {
                value:
                    CASValue::Finite {
                        digits: self_digits,
                        exp: self_exp,
                    },
                sign: self_sign,
            },
            CASNum {
                value:
                    CASValue::Finite {
                        digits: rhs_digits,
                        exp: rhs_exp,
                    },
                sign: rhs_sign,
            },
        ) = (&self, &rhs)
        {
            let (quot, ..) = division_finite(self_digits, self_exp, rhs_digits, rhs_exp);
            match (self_sign, rhs_sign) {
                (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => {
                    *self = quot;
                }
                (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => {
                    *self = -quot;
                } //a / -b = - (a / b)
                  //-a / b = -(a / b)
            };
        }
    }
}

fn quot_rem(lhs: &CASNum, rhs: u64) -> (CASNum, CASNum) {
    if lhs.value.is_zero() && rhs == 0 {
        //0/0 -> Nan
        (INDETERMINATE, INDETERMINATE)
    } else if lhs.value.is_zero() {
        //0/nonzero -> 0
        return (ZERO, ZERO);
    } else if lhs.value.is_indeterminate() {
        //nan/x -> nan
        return (INDETERMINATE, INDETERMINATE);
    } else if lhs.value.is_infinite() {
        //inf/x = inf
        return (INDETERMINATE, INDETERMINATE);
    } else if let CASNum {
        value: CASValue::Finite { digits, exp },
        sign,
    } = lhs
    {
        let mut rem: u128 = 0;
        let divisor = rhs as u128;
        let mut quot: VecDeque<u64> = VecDeque::from([]);
        for digit in digits.iter().rev() {
            let dividend = (*digit as u128) + (rem << 64);
            quot.push_front((dividend / divisor) as u64);
            rem = dividend % divisor;
        }

        return (
            CASNum {
                value: CASValue::Finite {
                    digits: quot,
                    exp: *exp,
                }
                .normalize(),
                sign: *sign,
            },
            CASNum::from(rem),
        );
    } else {
        unreachable!();
    }
}

impl DivAssign<u64> for CASNum {
    fn div_assign(&mut self, rhs: u64) {
        *self = quot_rem(self, rhs).0;
    }
}

impl RemAssign<u64> for CASNum {
    fn rem_assign(&mut self, rhs: u64) {
        *self = quot_rem(self, rhs).1;
    }
}

impl Div<u64> for CASNum {
    type Output = CASNum;

    fn div(mut self, rhs: u64) -> CASNum {
        self /= rhs;
        self
    }
}

impl Rem<u64> for CASNum {
    type Output = CASNum;

    fn rem(mut self, rhs: u64) -> Self::Output {
        self %= rhs;
        self
    }
}

fn division_finite(
    lhs_digits: &VecDeque<u64>,
    lhs_exp: &isize,
    rhs_digits: &VecDeque<u64>,
    rhs_exp: &isize,
) -> (CASNum, CASNum) {
    todo!();
}

// impl Add<&CASNum> for CASNum {
//     type Output = CASNum;

//     fn add(mut self, rhs: &CASNum) -> CASNum {
//         self += rhs;
//         self
//     }
// }

impl Add<CASNum> for CASNum {
    type Output = CASNum;

    fn add(mut self, rhs: CASNum) -> CASNum {
        self += rhs;
        self
    }
}

impl Sub<CASNum> for CASNum {
    type Output = CASNum;

    fn sub(mut self, rhs: CASNum) -> CASNum {
        self -= rhs;
        self
    }
}

impl Mul<CASNum> for CASNum {
    type Output = CASNum;

    fn mul(mut self, rhs: CASNum) -> CASNum {
        self *= rhs;
        self
    }
}

impl Div<CASNum> for CASNum {
    type Output = CASNum;

    fn div(mut self, rhs: CASNum) -> CASNum {
        self /= rhs;
        self
    }
}
