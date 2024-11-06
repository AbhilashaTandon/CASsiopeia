use std::{
    cmp::{max, min, Ordering},
    collections::VecDeque,
};

use super::CASNum;

pub fn align(a: &CASNum, b: &CASNum) -> VecDeque<(u8, u8, i128)> {
    //digits aligned by exponent and zipped together
    //base 10 example

    //1200, .003
    // (1, 0, 3) (2, 0, 2) (0, 0, 1) (0, 0, 0) . (0, 0, -1) (0, 0, -2) (0, 3, -3)
    //thousands place, hundreds place, tens place, ones place, tenths place, hundredths place, thousandths place
    let a_max_digit = a.max_digit();
    let a_min_digit = a.exp;
    let b_max_digit = b.max_digit();
    let b_min_digit = b.exp;
    let max_digit = max(a_max_digit, b_max_digit);
    let min_digit = min(a_min_digit, b_min_digit);

    let mut out: VecDeque<(u8, u8, i128)> = VecDeque::new();
    for i in min_digit..=max_digit {
        out.push_back((
            if a_min_digit <= i && i <= a_max_digit {
                a.bytes[(i - a_min_digit).try_into().unwrap()]
            } else {
                0
            },
            if b_min_digit <= i && i <= b_max_digit {
                b.bytes[(i - b_min_digit).try_into().unwrap()]
            } else {
                0
            },
            i,
        ));
    }

    return out;
}

pub fn cartesian(a: &CASNum, b: &CASNum) -> VecDeque<VecDeque<(u8, u8, i128)>> {
    //aligned cartesian product of base 256 digits
    //base 10 example

    //123.45, 4.567 ->
    // (1, 4,  2) (2, 4,  1) (3, 4,  0) . (4, 4, -1) (5, 4, -2)
    //     .      .      .        .      .
    // (1, 5,  1) (2, 5,  0) (3, 5, -1) . (4, 5, -2) (5, 5, -3)
    // (1, 6,  0) (2, 6, -1) (3, 6, -2) . (4, 6, -3) (5, 6, -4)
    // (1, 7, -1) (2, 7, -2) (3, 7, -3) . (4, 7, -4) (5, 7, -5)

    let a_max_digit = a.max_digit();
    let a_min_digit = a.exp;
    let b_max_digit = b.max_digit();
    let b_min_digit = b.exp;

    let mut out: VecDeque<VecDeque<(u8, u8, i128)>> = VecDeque::new();
    for i in a_min_digit..=a_max_digit {
        let mut row: VecDeque<(u8, u8, i128)> = VecDeque::new();
        for j in b_min_digit..=b_max_digit {
            row.push_back((
                if a_min_digit <= i && i <= a_max_digit {
                    a.bytes[(i - a_min_digit).try_into().unwrap()]
                } else {
                    0
                },
                if b_min_digit <= j && j <= b_max_digit {
                    b.bytes[(j - b_min_digit).try_into().unwrap()]
                } else {
                    0
                },
                i + j,
            ));
        }
        out.push_back(row);
    }
    return out;
}

pub fn compare(a: &CASNum, b: &CASNum) -> Ordering {
    //returns comparison of absolute values
    match a.bytes.len().cmp(&b.bytes.len()) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => {}
    }
    for (a_byte, b_byte, _) in align(a, b).iter().rev() {
        match a_byte.cmp(b_byte) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
        }
    }

    return Ordering::Equal;
}
