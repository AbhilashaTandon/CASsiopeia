//iterators

use super::{CASNum, Sign};

pub struct CASNumIter {
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
