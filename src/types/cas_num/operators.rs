//numerical operators, +, -, *, / etc
use std::{
    collections::VecDeque,
    ops::{self},
};

use crate::types::cas_num::{DigitType, NUM_BITS};

use super::{CASNum, CASValue, Sign, INDETERMINATE, INFINITY, NEG_INFINITY, ZERO};

impl ops::Neg for CASNum {
    type Output = CASNum;
    fn neg(self) -> Self::Output {
        return CASNum {
            value: self.value,
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
        match (self.clone(), rhs.clone()) {
            (
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
                            digits: other_digits,
                            exp: other_exp,
                        },
                    sign: rhs_sign,
                },
            ) => {
                let self_copy = CASNum {
                    value: CASValue::Finite {
                        digits: self_digits,
                        exp: self_exp,
                    },
                    sign: self_sign,
                };
                let rhs_copy = CASNum {
                    value: CASValue::Finite {
                        digits: other_digits,
                        exp: other_exp,
                    },
                    sign: rhs_sign,
                };

                return addition_finite(self_copy, rhs_copy);
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
        }
    }
}

fn addition_finite(lhs: CASNum, rhs: CASNum) -> CASNum {
    match (lhs.value.is_zero(), rhs.value.is_zero()) {
        (true, true) => return lhs,  //0 + 0 == 0
        (true, false) => return rhs, //0 + x == x
        (false, true) => return lhs, //x + 0 == x
        (false, false) => {}
    }
    match (lhs.sign, rhs.sign) {
        (Sign::Pos, Sign::Pos) => {
            let mut digits: VecDeque<DigitType> = VecDeque::new();
            let mut carry = 0;

            let self_value = lhs.value;
            let rhs_value = rhs.value;

            let alignment = self_value.align(&rhs_value).unwrap(); //we can unwrap safely since both self and rhs are finite

            let exp = alignment.front().unwrap().2;

            for (a_digit, b_digit, _) in alignment {
                let mut sum: u128 = a_digit as u128 + b_digit as u128 + carry;
                if sum > 0xFFFFFFFFFFFFFFFF {
                    carry = 1;
                    sum -= 0x10000000000000000;
                } else {
                    carry = 0;
                }
                let new_digit: DigitType = sum.try_into().unwrap();
                digits.push_back(new_digit);
            }

            if carry != 0 {
                digits.push_back(carry as DigitType);
            }

            return CASNum {
                value: CASValue::Finite { digits, exp },
                sign: Sign::Pos,
            };
        }

        (Sign::Pos, Sign::Neg) => lhs - rhs.abs(),
        (Sign::Neg, Sign::Pos) => rhs - lhs.abs(),
        (Sign::Neg, Sign::Neg) => -(lhs.abs() + rhs.abs()),
    }
}

impl ops::Sub<CASNum> for CASNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
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
                            digits: other_digits,
                            exp: other_exp,
                        },
                    sign: rhs_sign,
                },
            ) => {
                let self_copy = CASNum {
                    value: CASValue::Finite {
                        digits: self_digits,
                        exp: self_exp,
                    },
                    sign: self_sign,
                };
                let rhs_copy = CASNum {
                    value: CASValue::Finite {
                        digits: other_digits,
                        exp: other_exp,
                    },
                    sign: rhs_sign,
                };

                //TODO: find a better way than reconstructing CASdigits

                subtraction_finite(self_copy, rhs_copy)
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

fn subtraction_finite(lhs: CASNum, rhs: CASNum) -> CASNum {
    match (lhs.value.is_zero(), rhs.value.is_zero()) {
        (true, true) => return lhs,   //0 - 0 == 0
        (true, false) => return -rhs, //0 - x == x
        (false, true) => return lhs,  //x - 0 == x
        (false, false) => {}
    }
    match (lhs.sign, rhs.sign) {
        (Sign::Pos, Sign::Pos) => {
            if lhs < rhs {
                //a - b = -(b - a)
                return -(rhs - lhs);
            }
            let mut digits: VecDeque<DigitType> = VecDeque::new();
            let mut carry = 0;

            let self_value = lhs.value;
            let rhs_value = rhs.value;

            let alignment = self_value.align(&rhs_value).unwrap(); //we can unwrap safely since both self and rhs are finite

            let exp = alignment.front().unwrap().2;

            for (self_digit, other_digit, _) in alignment {
                let mut diff: i128 = (self_digit as i128) - (other_digit as i128) + carry;

                if diff < 0 {
                    diff = 0x10000000000000000 + diff;
                    carry = -1;
                } else {
                    carry = 0;
                }
                digits.push_back(diff.try_into().unwrap());
            }

            return CASNum {
                value: CASValue::Finite { digits, exp },
                sign: Sign::Pos,
            };
        }

        (Sign::Pos, Sign::Neg) => lhs + rhs.abs(), //a - -b = a + b
        (Sign::Neg, Sign::Pos) => -(rhs + lhs.abs()), ////-a - b = a - b
        (Sign::Neg, Sign::Neg) => rhs.abs() - lhs.abs(), //-a - -b = -a + b = b - a
    }
}

impl ops::Mul<CASNum> for CASNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.value.is_indeterminate() || rhs.value.is_indeterminate() {
            return INDETERMINATE;
            //NAN * x == NAN
            //x * NAN == NAN
        }

        if self.value.is_zero() && rhs.value.is_infinite() {
            return INDETERMINATE;

            //0 * +/- inf == NAN
        }

        if self.value.is_infinite() && rhs.value.is_zero() {
            return INDETERMINATE;

            // +/- inf  * 0 == NAN
        }

        match (self.sign, rhs.sign) {
            (Sign::Pos, Sign::Pos) => {}
            (Sign::Pos, Sign::Neg) => return -(self * (-rhs)),
            (Sign::Neg, Sign::Pos) => return -((-self) * rhs),
            (Sign::Neg, Sign::Neg) => return (-self) * (-rhs),
        };

        if self.value.is_infinite() || rhs.value.is_infinite() {
            return INFINITY; //we already checked sign so we can assume all beyond this point is positive
        }

        if self.value.is_zero() || rhs.value.is_zero() {
            return ZERO;
        }

        return multiplication_finite(self, rhs);
    }
}

fn multiplication_finite(lhs: CASNum, rhs: CASNum) -> CASNum {
    let cartesian = lhs.value.cartesian(&rhs.value).unwrap();

    println!("{:?} {:?}", lhs, rhs);

    let max_digit = cartesian.back().unwrap().back().unwrap().2;
    let min_digit = cartesian.front().unwrap().front().unwrap().2;

    assert!(max_digit >= min_digit);

    let mut temp_arr: Vec<u128> = vec![];
    for _ in min_digit..=max_digit {
        temp_arr.push(0);
    }

    for row in cartesian {
        for (self_digit, rhs_digit, exp) in row {
            temp_arr[(exp - min_digit) as usize] += (self_digit as u128) * (rhs_digit as u128);
        }
    }

    let mut carry = 0;
    let mut digits: VecDeque<DigitType> = VecDeque::new();

    for &value in temp_arr.iter() {
        let adjusted = value + carry;
        digits.push_back((adjusted & 0xFFFFFFFFFFFFFFFF) as DigitType);
        carry = adjusted >> NUM_BITS;
    }
    while carry > 0 {
        digits.push_back((carry & 0xFFFFFFFFFFFFFFFF).try_into().unwrap());
        carry >>= NUM_BITS;
    }

    return CASNum {
        value: CASValue::Finite {
            digits,
            exp: min_digit,
        }
        .normalize(),
        sign: Sign::Pos,
    };
}

impl ops::Div<CASNum> for CASNum {
    type Output = CASNum;

    fn div(self, rhs: CASNum) -> Self::Output {
        if self.value.is_indeterminate() || rhs.value.is_indeterminate() {
            return INDETERMINATE;
            //NAN / x == NAN
            //x / NAN == NAN
        }

        if self.value.is_zero() && rhs.value.is_zero() {
            return INDETERMINATE;
            //0/0 == NAN
        }

        if self.value.is_infinite() && rhs.value.is_infinite() {
            return INDETERMINATE;
        }

        match (self.sign, rhs.sign) {
            (Sign::Pos, Sign::Pos) => {}
            (Sign::Pos, Sign::Neg) => return -(self * (-rhs)),
            (Sign::Neg, Sign::Pos) => return -((-self) * rhs),
            (Sign::Neg, Sign::Neg) => return (-self) * (-rhs),
        };

        if rhs.value.is_infinite() {
            return INFINITY; //we already checked sign so we can assume all beyond this point is positive
        }

        if self.value.is_zero() || rhs.value.is_infinite() {
            return ZERO;
        }

        return division_finite(self, rhs);
    }
}

fn division_finite(lhs: CASNum, rhs: CASNum) -> CASNum {
    todo!();
}
