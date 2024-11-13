//comparison traits

use std::cmp::Ordering;

use super::{
    CASNum,
    CASValue::{self, Finite, Indeterminate, Infinite},
    Sign,
};

impl PartialEq<CASNum> for CASNum {
    fn eq(&self, other: &CASNum) -> bool {
        //this works like floating point
        //0 == -0, infinity > finite numbers, indeterminate != indeterminate (equiv of nan)

        match (self.value.is_zero(), other.value.is_zero()) {
            (true, true) => return true,   //0 == -0
            (true, false) => return false, //0 != nonzero
            (false, true) => return false, //0 != nonzero
            (false, false) => {}
        }

        match (self.sign, other.sign) {
            (Sign::Pos, Sign::Pos) => {}
            (Sign::Pos, Sign::Neg) => return false, //nonzero != -nonzero
            (Sign::Neg, Sign::Pos) => return false, //nonzero != -nonzero
            (Sign::Neg, Sign::Neg) => {}
        }

        match (&self.value, &other.value) {
            (
                Finite {
                    digits: self_digits,
                    exp: self_exp,
                },
                Finite {
                    digits: other_digits,
                    exp: other_exp,
                },
            ) => {
                //this will not work if values aren't normalized
                //so we must ensure theyre normalized after all possible operations
                self_digits == other_digits && self_exp == other_exp
                //at this point we've ensured that self and other are two finite nonzero numbers with the same sign
            }
            (Finite { .. }, Infinite) => false,
            (Finite { .. }, Indeterminate) => false,
            (Infinite, Finite { .. }) => false,
            (Infinite, Infinite) => true, //we know signs are the same
            (Infinite, Indeterminate) => false,
            (Indeterminate, Finite { .. }) => false,
            (Indeterminate, Infinite) => false,
            (Indeterminate, Indeterminate) => false,
        }
    }
}

use std::cmp::Ordering::{Equal, Greater, Less};

impl PartialOrd for CASNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Equal); //ensures PartialOrd is consistent w PartialEq
        }
        match (self, other) {
            (
                CASNum {
                    value: CASValue::Finite { .. },
                    ..
                },
                CASNum {
                    value: CASValue::Finite { .. },
                    ..
                },
            ) => Some(self.compare_finite(other)),
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
                (_, Sign::Pos) => Some(Less),    //finite < inf
                (_, Sign::Neg) => Some(Greater), //finite > -inf
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
                (Sign::Pos, _) => Some(Greater), //inf > finite
                (Sign::Neg, _) => Some(Less),    //-inf < finite
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
                (Sign::Pos, Sign::Pos) => Some(Equal),   //inf == inf
                (Sign::Pos, Sign::Neg) => Some(Greater), //inf > -inf
                (Sign::Neg, Sign::Pos) => Some(Less),    //-inf < inf
                (Sign::Neg, Sign::Neg) => Some(Equal),   //-inf == -inf,
            },
            _ => None, //indeterminate (NaN) is uncomparable
        }
    }
}
