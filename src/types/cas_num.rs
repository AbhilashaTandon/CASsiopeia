use std::cmp::max;
use std::ops;
use std::path::Iter;

#[derive(Debug, PartialEq, Eq)]
enum Sign {
    Pos,
    Neg,
}

#[derive(Debug)]
struct CASNum {
    bytes: Vec<u8>, //little endian
    exp: i128,
    sign: Sign,
}

struct CASNumIter {
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

fn normalize(n: &mut CASNum) {
    while let Some(least_order_byte) = n.bytes.get(0) {
        if least_order_byte & 1 == 0 {
            //if has trailing 0
            n.exp += 1;
            let mut remainder = 0;
            let shift_left = n.bytes.iter().enumerate().rev().map(|(idx, byte)| {
                let shl = byte >> 1 + remainder << 7;
                remainder = byte & 1;
                eprint!("{} ", shl);
                shl
            });
            eprintln!();
        }
    }
}
