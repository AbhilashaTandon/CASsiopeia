//numerical operators, +, -, *, / etc
use std::{collections::VecDeque, ops};

use super::{CASNum, CASValue, Sign, INDETERMINATE, INFINITY, NEG_INFINITY};

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

                addition_float(self_copy, rhs_copy)
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

fn addition_float(lhs: CASNum, rhs: CASNum) -> CASNum {
    match (lhs.sign, rhs.sign) {
        (Sign::Pos, Sign::Pos) => {
            let mut bytes: VecDeque<u8> = VecDeque::new();
            let mut carry = 0;

            let self_value = lhs.value;
            let rhs_value = rhs.value;

            let alignment = self_value.align(&rhs_value).unwrap(); //we can unwrap safely since both self and rhs are finite

            for (a_byte, b_byte, _) in alignment {
                let sum: u16 = a_byte as u16 + b_byte as u16 + carry;
                if sum >= 256 {
                    carry = sum / 256;
                }
                let new_byte: u8 = (sum % 256).try_into().unwrap();
                bytes.push_back(new_byte);
            }

            return CASNum {
                value: CASValue::Finite { bytes, exp: 0 },
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

                subtraction_float(self_copy, rhs_copy)
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

fn subtraction_float(lhs: CASNum, rhs: CASNum) -> CASNum {
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

            for (self_byte, other_byte, _) in alignment {
                let mut diff: i16 = (self_byte as i16) - (other_byte as i16) - carry;

                if diff < 0 {
                    diff = 255 + diff;
                    carry = 1;
                }
                bytes.push_back(diff.try_into().unwrap());
            }

            return CASNum {
                value: CASValue::Finite { bytes, exp: 0 },
                sign: Sign::Pos,
            };
        }

        (Sign::Pos, Sign::Neg) => lhs + rhs.abs(), //a - -b = a + b
        (Sign::Neg, Sign::Pos) => -(rhs + lhs.abs()), ////-a - b = a - b
        (Sign::Neg, Sign::Neg) => rhs.abs() - lhs.abs(), //-a - -b = -a + b = b - a
    }
}
