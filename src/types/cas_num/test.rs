#[cfg(test)]
use std::collections::VecDeque;

use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::types::cas_num::{CASValue::*, Sign::*};

use super::super::CASNum;
use super::literal;

fn comparison(a: i128, b: i128) -> bool {
    CASNum::from(a).partial_cmp(&CASNum::from(b)) == Some(a.cmp(&b))
}

fn addition(a: i128, b: i128) -> bool {
    let _a_casnum = CASNum::from(a);
    let _b_casnum = CASNum::from(b);
    let mut sum_1 = CASNum::from(a + b);
    let mut sum_2 = CASNum::from(a) + &CASNum::from(b);
    sum_1.value = sum_1.value.normalize();
    sum_2.value = sum_2.value.normalize();
    sum_1 == sum_2
}

fn subtraction(a: i128, b: i128) -> bool {
    let mut sum_1 = CASNum::from(a - b);
    let mut sum_2 = CASNum::from(a) - &CASNum::from(b);
    sum_1.value = sum_1.value.normalize();
    sum_2.value = sum_2.value.normalize();
    sum_1 == sum_2
}

fn multiplication(a: i128, b: i128) -> bool {
    let prod_1 = CASNum::from(a * b);
    let prod_2 = CASNum::from(a) * &CASNum::from(b);
    print!("{:?}\t{:?}\t", prod_1, prod_2,);
    prod_1 == prod_2
}

fn comparison_float(a: f32, b: f32) -> bool {
    let casnum_a = CASNum::from(a);
    let casnum_b = CASNum::from(b);
    casnum_a.partial_cmp(&casnum_b) == a.partial_cmp(&b)
}

fn addition_float(a: f64, b: f64, result: CASNum) -> bool {
    let mut sum = CASNum::from(a) + &CASNum::from(b);
    sum.value = sum.value.normalize();
    if sum != result {
        return false;
    }

    sum = CASNum::from(a as f32) + &CASNum::from(b as f32);
    sum.value = sum.value.normalize();

    if sum != result {
        return false;
    }
    true
}

fn subtraction_float(a: f64, b: f64, result: CASNum) -> bool {
    let mut sum = CASNum::from(a) - &CASNum::from(b);
    sum.value = sum.value.normalize();

    if sum != result {
        return false;
    }

    sum = CASNum::from(a as f32) - &CASNum::from(b as f32);
    sum.value = sum.value.normalize();
    if sum != result {
        return false;
    }
    true
}

fn test_conversion(value: f64, desired_output_32: CASNum, desired_output_64: CASNum) -> bool {
    let thirty_two_bit = value as f32;
    let sixty_four_bit = value;
    if CASNum::from(thirty_two_bit) != desired_output_32 {
        return false;
    }
    if CASNum::from(sixty_four_bit) != desired_output_64 {
        return false;
    }
    true
}

#[test]
fn conversion_tests() {
    assert_eq!(
        CASNum::from(1),
        CASNum {
            value: Finite {
                digits: VecDeque::from([1]),
                exp: 0
            },
            sign: Pos,
        },
    );
    assert_eq!(
        CASNum::from(-1),
        CASNum {
            value: Finite {
                digits: VecDeque::from([1]),
                exp: 0
            },
            sign: Neg,
        },
    );
    assert_eq!(
        CASNum::from(523563),
        CASNum {
            value: Finite {
                digits: VecDeque::from([523563]),
                exp: 0
            },
            sign: Pos,
        },
    );
    assert_eq!(
        CASNum::from(6531),
        CASNum {
            value: Finite {
                digits: VecDeque::from([6531]),
                exp: 0
            },
            sign: Pos,
        },
    );

    assert_eq!(
        CASNum::from(154),
        CASNum {
            value: Finite {
                digits: VecDeque::from([154]),
                exp: 0
            },
            sign: Pos,
        },
    );
    assert_eq!(
        CASNum::from(145),
        CASNum {
            value: Finite {
                digits: VecDeque::from([145]),
                exp: 0
            },
            sign: Pos,
        },
    );
}

#[test]
fn conversion_tests_float() {
    assert!(test_conversion(
        0.0000019073486328125,
        CASNum {
            value: Finite {
                digits: VecDeque::from([35184372088832]),
                exp: -1,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([35184372088832]),
                exp: -1,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        -2.348_443_963_552_745_6e-22,
        CASNum {
            value: Finite {
                digits: VecDeque::from([79913407049891840]),
                exp: -2,
            },
            sign: Neg,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([79913407049891840]),
                exp: -2,
            },
            sign: Neg,
        },
    ));
    assert!(test_conversion(
        1.040_913_616_315_288_6e-27,
        CASNum {
            value: Finite {
                digits: VecDeque::from([354204549120]),
                exp: -2,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([354204549120]),
                exp: -2,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        -1.839_960_072_688_999_6e31,
        CASNum {
            value: Finite {
                digits: VecDeque::from([997444354048]),
                exp: 1,
            },
            sign: Neg,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([997444354048]),
                exp: 1,
            },
            sign: Neg,
        },
    ));
    assert!(test_conversion(
        0.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Neg,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Neg,
        },
    ));
    assert!(test_conversion(
        902341.2532,
        CASNum {
            value: Finite {
                digits: VecDeque::from([4611686018427387904, 902341]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([4670715600119529472, 902341]),
                exp: 0,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        0239402.2340923,
        CASNum {
            value: Finite {
                digits: VecDeque::from([4323455642275676160, 0239402]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([4318240747745705984, 0239402]),
                exp: 0,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        55.592_082_977_294_92,
        CASNum {
            value: Finite {
                digits: VecDeque::from([10922003152559407104, 55]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([10922003152559407104, 55]),
                exp: 0,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        1.956_046_964_696_149_4e-234,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([56019041081957608]),
                exp: -13,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        6.2938409230490e102,
        CASNum {
            value: Infinite,
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([11491643866163970048, 2946572]),
                exp: 5,
            },
            sign: Pos,
        },
    ));
    assert!(test_conversion(
        0.5,
        CASNum {
            value: Finite {
                digits: VecDeque::from([9223372036854775808]),
                exp: -1,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([9223372036854775808]),
                exp: -1,
            },
            sign: Pos,
        },
    ));

    assert!(test_conversion(
        1.040_913_616_315_288_6e-27,
        CASNum {
            value: Finite {
                digits: VecDeque::from([354204549120]),
                exp: -2,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([354204549120]),
                exp: -2,
            },
            sign: Pos,
        },
    ));

    assert!(test_conversion(
        5.0653025104231136e-102,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([576460752303423488, 199583080902304]),
                exp: -6,
            },
            sign: Pos,
        },
    ));

    assert!(test_conversion(
        7.891780781773763e127,
        CASNum {
            value: Infinite,
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([13344165695898779648, 2002888061698]),
                exp: 6,
            },
            sign: Pos,
        },
    ));

    assert!(test_conversion(
        1.4186493757725223e-209,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([17822545243984101376, 1193]),
                exp: -11,
            },
            sign: Pos,
        },
    ));

    assert!(test_conversion(
        5.879_627_713_392_078e-23,
        CASNum {
            value: Finite {
                digits: VecDeque::from([20007336349270016]),
                exp: -2,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([20007336349270016]),
                exp: -2,
            },
            sign: Pos,
        },
    ));

    assert!(test_conversion(
        56173.81640625,
        CASNum {
            value: Finite {
                digits: VecDeque::from([15060037153926938624, 56173]),
                exp: 0,
            },
            sign: Pos,
        },
        CASNum {
            value: Finite {
                digits: VecDeque::from([15060037153926938624, 56173]),
                exp: 0,
            },
            sign: Pos,
        },
    ));
}

#[test]
fn addition_tests() {
    assert!(addition(1, 0));
    assert!(addition(0, 0));
    assert!(addition(0, 1));
    assert!(addition(-1, 0));
    assert!(addition(0, -1));
    assert!(addition(-1, -1));
    assert!(addition(1, -1));
    assert!(addition(-1, 1));
    assert!(addition(1, 1));

    assert!(addition(12032, 23420));
    assert!(addition(02312, 054123));
    assert!(addition(012312, 11231));
    assert!(addition(-52521, 01231));
    assert!(addition(10532153, -11252));
    assert!(addition(-235131, -65347641));
    assert!(addition(46589611, -15489456));
    assert!(addition(-541, 154));
    assert!(addition(154, 145));
}
#[test]
fn subtraction_tests() {
    assert!(subtraction(1, 0));
    assert!(subtraction(0, 0));
    assert!(subtraction(0, 1));
    assert!(subtraction(-1, 0));
    assert!(subtraction(0, -1));
    assert!(subtraction(-1, -1));
    assert!(subtraction(1, -1));
    assert!(subtraction(-1, 1));
    assert!(subtraction(1, 1));

    assert!(subtraction(12032, 23420));
    assert!(subtraction(02312, 054123));

    assert!(subtraction(012312, 11231));
    assert!(subtraction(-52521, 01231));
    assert!(subtraction(10532153, -11252));
    assert!(subtraction(-235131, -65347641));
    assert!(subtraction(46589611, -15489456));
    assert!(subtraction(-541, 154));
    assert!(subtraction(154, 145));
}

#[test]
fn comparison_tests() {
    assert!(comparison(1, 0));
    assert!(comparison(0, 0));
    assert!(comparison(0, 1));
    assert!(comparison(-1, 0));
    assert!(comparison(0, -1));
    assert!(comparison(-1, -1));
    assert!(comparison(1, -1));
    assert!(comparison(-1, 1));
    assert!(comparison(1, 1));

    assert!(comparison(12032, 23420));
    assert!(comparison(02312, 054123));
    assert!(comparison(012312, 11231));
    assert!(comparison(-52521, 01231));
    assert!(comparison(10532153, -11252));
    assert!(comparison(-235131, -65347641));
    assert!(comparison(46589611, -15489456));
    assert!(comparison(-541, 154));
    assert!(comparison(154, 145));
}

#[test]
fn comparison_float_tests() {
    let floats_of_choice: Vec<f32> = vec![
        -2.348_444e-22,
        1.040_913_6e-27,
        -1.839_960_1e31,
        0.,
        902_341.25,
        239_402.23,
        09.3423,
        -0.003_042_049_2,
    ];

    for a in &floats_of_choice {
        for b in &floats_of_choice {
            assert!(comparison_float(*a, *b));
        }
    }
}
#[test]
fn addition_float_tests() {
    assert!(addition_float(
        0.,
        0.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(addition_float(
        0.,
        1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([1]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(addition_float(
        0.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([1]),
                exp: 0,
            },
            sign: Neg,
        },
    ));

    assert!(addition_float(
        1.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Neg,
        },
    ));

    assert!(addition_float(
        1.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([0]),
                exp: 0,
            },
            sign: Neg,
        },
    ));

    assert!(addition_float(
        100.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([99]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(addition_float(
        f64::INFINITY,
        f64::INFINITY,
        CASNum {
            value: Infinite,
            sign: Pos,
        },
    ));

    assert!(addition_float(
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        CASNum {
            value: Infinite,
            sign: Neg,
        },
    ));

    assert!(addition_float(
        55.592_082_977_294_92,
        13.384_548_187_255_86,
        CASNum {
            value: Finite {
                digits: VecDeque::from([18015665146877181952, 68]),
                exp: 0,
            },
            sign: Pos,
        },
    ));
}

#[test]
fn subtraction_float_tests() {
    assert!(subtraction_float(
        0.,
        0.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(subtraction_float(
        0.,
        1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([1]),
                exp: 0,
            },
            sign: Neg,
        },
    ));

    assert!(subtraction_float(
        0.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([1]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(subtraction_float(
        1.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([2]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(subtraction_float(
        100.,
        -1.,
        CASNum {
            value: Finite {
                digits: VecDeque::from([101]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(subtraction_float(
        55.592_082_977_294_92,
        13.384_548_187_255_86,
        CASNum {
            value: Finite {
                digits: VecDeque::from([3828341158241632256, 42]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(subtraction_float(
        13.384_548_187_255_86,
        55.592_082_977_294_92,
        CASNum {
            value: Finite {
                digits: VecDeque::from([3828341158241632256, 42]),
                exp: 0,
            },
            sign: Neg,
        },
    ));

    assert!(subtraction_float(
        -55.592_082_977_294_92,
        13.384_548_187_255_86,
        CASNum {
            value: Finite {
                digits: VecDeque::from([18015665146877181952, 68]),
                exp: 0,
            },
            sign: Neg,
        },
    ));

    assert!(subtraction_float(
        13.384_548_187_255_86,
        -55.592_082_977_294_92,
        CASNum {
            value: Finite {
                digits: VecDeque::from([18015665146877181952, 68]),
                exp: 0,
            },
            sign: Pos,
        },
    ));

    assert!(subtraction_float(
        5.879_627_713_392_078e-23,
        56173.81640625,
        CASNum {
            value: Finite {
                digits: VecDeque::from([18426736737360281600, 15060037153926938623, 56173]),
                exp: 0,
            },
            sign: Neg,
        },
    ));
}

#[test]
fn multiplication_tests() {
    let values_to_test: Vec<i128> = vec![
        0,
        1,
        -1,
        23,
        -62,
        2853,
        -9584,
        84975630,
        -57993053,
        5799375739574939,
        -58399398578248,
    ];
    let mut num_wrong = 0;

    for value_1 in &values_to_test {
        for value_2 in &values_to_test {
            if !multiplication(*value_1, *value_2) {
                num_wrong += 1;
            }
        }
    }
    assert_eq!(num_wrong, 0);
}

#[test]

fn conversion_tests_reverse() {
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let mut num_wrong = 0;
    let limit = 10000;
    for _ in 0..limit {
        let bits: u64 = rng.next_u64();

        let float: f64 = f64::from_bits(bits);
        if float.is_nan() {
            continue;
        }
        //
        let cas_num = CASNum::from(float);
        //
        let reconstructed: f64 = cas_num.into();
        if reconstructed != float {
            num_wrong += 1;
        }
    }
    println!("{} / {}", limit - num_wrong, limit);
    assert_eq!(num_wrong, 0);
}

#[test]
fn literal_parsing() {
    let num_lits = vec![
            "9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
            // "3",
            // ".3",
            // "1.3",
            // "-3",
            // "-.3",
            // "-1.3",
            // "1.-3",
            // "12a",
            // "1,2.a",
            // "+-12,2.54",
            // "02839583928934294234",
            // "e3",
            // "e.3",
            // "e3.3",
            // "e-3",
            // "e-.3",
            // "e-3.3",
            // "3e1",
            // ".3e1",
            // "1.3e1",
            // "-3e1",
            // "-.3e1",
            // "-1.3e1",
            // "1.-3e1",
            // "12ae1",
            // "1,2.ae1",
            // "+-12,2.54e1",
            // "02839583928934294234e1",
            // "3e-1",
            // ".3e-1",
            // "1.3e-1",
            // "-3e-1",
            // "-.3e-1",
            // "-1.3e-1",
            // "1.-3e-1",
            // "12ae-1",
            // "1,2.ae-1",
            // "+-12,2.54e-1",
            // "02839583928934294234e-1",
        ];

    for lit in num_lits {
        let mut iter = lit.chars().enumerate().peekable();
        let first_char = iter.next().unwrap().1;
        literal::parse_lit(first_char, &mut iter, &mut 0);
    }
}

#[test]
fn multiplication_int_tests() {
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let mut num_wrong = 0;
    let limit = 10000;
    for _ in 0..limit {
        let int_1: u64 = rng.next_u32() as u64;
        let int_2: u64 = rng.next_u32() as u64;

        let rust_product = CASNum::from(int_1 * int_2);
        let my_product = CASNum::from(int_1) * &CASNum::from(int_2);

        if rust_product != my_product {
            num_wrong += 1;
        }
    }
    println!("{} / {}", limit - num_wrong, limit);
    assert_eq!(num_wrong, 0);
}

#[test]
fn multiplication_float_tests() {
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let mut num_wrong = 0;
    let limit = 10000;
    for _ in 0..limit {
        let float_1 = f32::from_bits(rng.next_u32()) as f64;
        let float_2 = f32::from_bits(rng.next_u32()) as f64;

        let casnum_1 = CASNum::from(float_1);
        let casnum_2 = CASNum::from(float_2);

        let rust_product = CASNum::from(float_1 * float_2);
        let my_product = casnum_1.clone() * &casnum_2;

        if float_1.is_nan() || float_2.is_nan() {
            if !my_product.value.is_indeterminate() {
                num_wrong += 1;
            } else {
                continue;
            }
        }

        // println!("-----------------------------------------------------------");
        if rust_product != my_product {
            print!(
                "{:e}\t{:e}\t{:?}\t{:?}\t{:?}\t{:?}",
                float_1,
                float_2,
                casnum_1.value,
                casnum_2.value,
                rust_product.value.exp().unwrap(),
                my_product.value.exp().unwrap()
            );
            // println!("{:e} {:e}", float_1, float_2);

            // println!(
            //     "cartesian: \n{:?}\n",
            //     &casnum_1.value.cartesian(&casnum_2.value)
            // );
            // println!(
            //     "input 1: {:?}\ninput 2: {:?}\n\n",
            //     CASNum::from(float_1),
            //     CASNum::from(float_2)
            // );
            // let my_product = CASNum::from(float_1) * &CASNum::from(float_2);
            // assert_eq!(rust_product, my_product);
            // println!("desired: {:?}\nreceived: {:?}", rust_product, my_product);
            num_wrong += 1;
            // assert!(false);
        }
        println!();
    }
    println!("\n\n{} / {}", limit - num_wrong, limit);
    assert_eq!(num_wrong, 0);
}

#[test]
fn div_single_tests() {
    let mut num_wrong = 0;
    let nums: Vec<u64> = vec![
        1,
        93,
        09794,
        88,
        56497,
        6648564,
        97367464849,
        9794758579457499499,
    ];
    for num_1 in &nums {
        for num_2 in &nums {
            let divisor = *num_1;

            let dividend = CASNum::from(*num_2);
            let quotient = dividend.clone() / divisor;
            let remainder = dividend.clone() % divisor;

            let reconstructed = quotient.clone() * &CASNum::from(divisor) + &remainder;
            if dividend != reconstructed {
                num_wrong += 1;
            }
        }
    }
    assert_eq!(num_wrong, 0);
}
#[test]
fn div_by_u64() {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let limit = 10000;
    let mut num_wrong = 0;
    for _ in 0..limit {
        let num_1 = CASNum::from(rng.next_u64());
        let num_2 = CASNum::from(rng.next_u64());
        let num_3 = CASNum::from(rng.next_u64());
        let divisor = rng.next_u64();

        let dividend = num_1 * &num_2 * &num_3;
        let quotient = dividend.clone() / divisor;
        let remainder = dividend.clone() % divisor;

        if dividend != (quotient.clone() * &CASNum::from(divisor) + &remainder) {
            num_wrong += 1;
        }
    }
    assert_eq!(num_wrong, 0);
}
