//comparison traits

use std::cmp::Ordering;

use super::{helper, CASNum, Sign};

impl PartialEq<CASNum> for CASNum {
    fn eq(&self, other: &CASNum) -> bool {
        let mut norm_self = <CASNum as Clone>::clone(&self);
        let mut norm_other = <CASNum as Clone>::clone(&other);
        norm_self.normalize();
        norm_other.normalize();

        norm_self.bytes == norm_other.bytes
            && norm_self.exp == norm_other.exp
            && norm_self.sign == norm_other.sign
    }
}

impl Eq for CASNum {} //inherits from PartialEq i think
impl PartialOrd for CASNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sign == Sign::Pos && other.sign == Sign::Pos {
            return Some(helper::compare(self, other));
        } else if self.sign == Sign::Neg && other.sign == Sign::Pos {
            //-a < b
            return Some(Ordering::Less);
        } else if self.sign == Sign::Pos && other.sign == Sign::Neg {
            //a > -b
            return Some(Ordering::Greater);
        } else {
            //if both negative, -a < -b == b < a
            return Some(helper::compare(other, self));
        }
    }
}
