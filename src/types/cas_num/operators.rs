//numerical operators, +, -, *, / etc
use std::{
    collections::VecDeque,
    ops::{self},
};

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
        match (self, rhs) {
            (
                CASNum {
                    value:
                        CASValue::Finite {
                            bytes: self_bytes,
                            exp: self_exp,
                        },
                    sign: self_sign,
                },
                CASNum {
                    value:
                        CASValue::Finite {
                            bytes: other_bytes,
                            exp: other_exp,
                        },
                    sign: rhs_sign,
                },
            ) => {
                let self_copy = CASNum {
                    value: CASValue::Finite {
                        bytes: self_bytes,
                        exp: self_exp,
                    },
                    sign: self_sign,
                };
                let rhs_copy = CASNum {
                    value: CASValue::Finite {
                        bytes: other_bytes,
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
            let mut bytes: VecDeque<u8> = VecDeque::new();
            let mut carry = 0;

            let self_value = lhs.value;
            let rhs_value = rhs.value;

            let alignment = self_value.align(&rhs_value).unwrap(); //we can unwrap safely since both self and rhs are finite

            let exp = alignment.front().unwrap().2;

            for (a_byte, b_byte, _) in alignment {
                let mut sum: u16 = a_byte as u16 + b_byte as u16 + carry;
                if sum >= 256 {
                    carry = 1;
                    sum -= 256;
                } else {
                    carry = 0;
                }
                let new_byte: u8 = sum.try_into().unwrap();
                bytes.push_back(new_byte);
            }

            if carry != 0 {
                bytes.push_back(carry as u8);
            }

            return CASNum {
                value: CASValue::Finite { bytes, exp },
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
                            bytes: self_bytes,
                            exp: self_exp,
                        },
                    sign: self_sign,
                },
                CASNum {
                    value:
                        CASValue::Finite {
                            bytes: other_bytes,
                            exp: other_exp,
                        },
                    sign: rhs_sign,
                },
            ) => {
                let self_copy = CASNum {
                    value: CASValue::Finite {
                        bytes: self_bytes,
                        exp: self_exp,
                    },
                    sign: self_sign,
                };
                let rhs_copy = CASNum {
                    value: CASValue::Finite {
                        bytes: other_bytes,
                        exp: other_exp,
                    },
                    sign: rhs_sign,
                };

                //TODO: find a better way than reconstructing CASnums

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
            let mut bytes: VecDeque<u8> = VecDeque::new();
            let mut carry: i16 = 0;

            let self_value = lhs.value;
            let rhs_value = rhs.value;

            let alignment = self_value.align(&rhs_value).unwrap(); //we can unwrap safely since both self and rhs are finite

            let exp = alignment.front().unwrap().2;

            for (self_byte, other_byte, _) in alignment {
                let mut diff: i16 = (self_byte as i16) - (other_byte as i16) + carry;

                if diff < 0 {
                    diff = 256 + diff;
                    carry = -1;
                } else {
                    carry = 0;
                }
                bytes.push_back(diff.try_into().unwrap());
            }

            return CASNum {
                value: CASValue::Finite { bytes, exp },
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

    let max_digit = cartesian.back().unwrap().back().unwrap().2;
    let min_digit = cartesian.front().unwrap().front().unwrap().2;

    assert!(max_digit >= min_digit);

    let mut temp_arr: Vec<u32> = vec![];
    for _ in min_digit..=max_digit {
        temp_arr.push(0);
    }

    for row in cartesian {
        for (self_byte, rhs_byte, exp) in row {
            temp_arr[(exp - min_digit) as usize] += (self_byte as u32) * (rhs_byte as u32);
        }
    }

    let mut carry = 0;
    let mut bytes: VecDeque<u8> = VecDeque::new();

    for &value in temp_arr.iter() {
        let adjusted = value + carry;
        bytes.push_back((adjusted % 256) as u8);
        carry = adjusted / 256;
    }
    while carry > 0 {
        bytes.push_back((carry % 256).try_into().unwrap());
        carry /= 256;
    }

    return CASNum {
        value: CASValue::Finite {
            bytes,
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
