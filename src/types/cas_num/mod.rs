// use std::cmp::max;
// use std::ops;
// use std::path::Iter;

use std::collections::VecDeque;

use conversion::CASNumConvert;

mod comp;
mod conversion;
mod helper;
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
    pub(crate) bytes: VecDeque<u8>, //little endian
    pub(crate) exp: i128,           //base 256
    pub(crate) sign: Sign,
}

impl CASNum {
    pub(crate) fn new<T: CASNumConvert>(i: T) -> CASNum {
        return i.to_cas_num();
    }

    fn max_digit(self: &Self) -> i128 {
        //exponent position of first digit
        return (self.bytes.len() as i128) - 1 + self.exp;
    }

    pub(crate) fn normalize(self: &mut Self) {
        while let Some(least_order_byte) = self.bytes.front() {
            if *least_order_byte == 0 {
                //if has trailing 0 remove it
                self.exp += 1;
                self.bytes.pop_front();
            } else {
                break;
            }
        }
    }

    pub(crate) fn abs(self) -> CASNum {
        return CASNum {
            bytes: self.bytes,
            exp: self.exp,
            sign: Sign::Pos,
        };
    }
}
