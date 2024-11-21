#[cfg(test)]
mod test {
    use core::num;
    use std::collections::VecDeque;

    use crate::types::cas_num::literal;
    use crate::types::cas_num::Sign;

    use crate::types::cas_num::CASValue::Finite;
    use crate::types::cas_num::CASValue::Infinite;

    use super::super::CASNum;

    use rand::RngCore;

    fn comparison(a: i128, b: i128) {
        assert_eq!(CASNum::from(a) < CASNum::from(b), a < b);
        assert_eq!(CASNum::from(a) > CASNum::from(b), a > b);
        assert_eq!(CASNum::from(a) == CASNum::from(b), a == b);
    }

    fn addition(a: i128, b: i128) {
        let _a_casnum = CASNum::from(a);
        let _b_casnum = CASNum::from(b);
        let mut sum_1 = CASNum::from(a + b);
        let mut sum_2 = CASNum::from(a) + &CASNum::from(b);
        sum_1.value = sum_1.value.normalize();
        sum_2.value = sum_2.value.normalize();
        assert_eq!(sum_1, sum_2);
    }

    fn subtraction(a: i128, b: i128) {
        let mut sum_1 = CASNum::from(a - b);
        let mut sum_2 = CASNum::from(a) - &CASNum::from(b);
        sum_1.value = sum_1.value.normalize();
        sum_2.value = sum_2.value.normalize();
        assert_eq!(sum_1, sum_2);
    }

    fn multiplication(a: i128, b: i128) {
        let prod_1 = CASNum::from(a * b);
        let prod_2 = CASNum::from(a) * &CASNum::from(b);
        assert_eq!(prod_1, prod_2);
    }

    fn comparison_float(a: f32, b: f32) {
        let casnum_a = CASNum::from(a);
        let casnum_b = CASNum::from(b);
        assert_eq!(casnum_a < casnum_b, a < b);
        assert_eq!(casnum_a > casnum_b, a > b);
        assert_eq!(casnum_a == casnum_b, a == b);
    }

    fn addition_float(a: f64, b: f64, result: CASNum) {
        let mut sum = CASNum::from(a) + &CASNum::from(b);
        sum.value = sum.value.normalize();
        if sum != result {
            println!("64 bit");
            println!("{:e} {:e}", a, b);
            println!("{:?} {:?}", sum, result);
            assert!(false);
        }
        assert_eq!(sum, result);

        sum = CASNum::from(a as f32) + &CASNum::from(b as f32);
        sum.value = sum.value.normalize();

        if sum != result {
            println!("32 bit");
            println!("{:e} {:e}", a, b);
            println!("{:?} {:?}", sum, result);
            assert!(false);
        }
        assert_eq!(sum, result);
        assert_eq!(sum, result);
    }

    fn subtraction_float(a: f64, b: f64, result: CASNum) {
        let mut sum = CASNum::from(a) - &CASNum::from(b);
        sum.value = sum.value.normalize();

        if sum != result {
            println!("{:?} {:?}", sum, result);
            println!("64 bit");
            println!("{:e} {:e}", a, b);
            assert!(false);
        }

        sum = CASNum::from(a as f32) - &CASNum::from(b as f32);
        sum.value = sum.value.normalize();
        if sum != result {
            println!("32 bit");
            println!("{:e} {:e}", a, b);
            assert!(false);
        }
    }

    fn test_conversion(value: f64, desired_output_32: CASNum, desired_output_64: CASNum) {
        let thirty_two_bit = value as f32;
        let sixty_four_bit = value as f64;
        if CASNum::from(thirty_two_bit) != desired_output_32 {
            println!("32 bit");
            println!("value: {:e}", thirty_two_bit);
            println!(
                "actual value: {:?}\ngiven value: {:?}",
                desired_output_32,
                CASNum::from(thirty_two_bit)
            );
            assert!(false);
        }
        if CASNum::from(sixty_four_bit) != desired_output_64 {
            println!("64 bit");
            println!("value: {:e}", sixty_four_bit);
            println!(
                "actual value: {:?}\ngiven value: {:?}",
                desired_output_64,
                CASNum::from(sixty_four_bit)
            );
            assert!(false);
        }
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
                sign: Sign::Pos,
            },
        );
        assert_eq!(
            CASNum::from(-1),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([1]),
                    exp: 0
                },
                sign: Sign::Neg,
            },
        );
        assert_eq!(
            CASNum::from(523563),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([523563]),
                    exp: 0
                },
                sign: Sign::Pos,
            },
        );
        assert_eq!(
            CASNum::from(6531),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([6531]),
                    exp: 0
                },
                sign: Sign::Pos,
            },
        );

        assert_eq!(
            CASNum::from(154),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([154]),
                    exp: 0
                },
                sign: Sign::Pos,
            },
        );
        assert_eq!(
            CASNum::from(145),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([145]),
                    exp: 0
                },
                sign: Sign::Pos,
            },
        );
    }

    #[test]
    fn conversion_tests_float() {
        test_conversion(
            0.0000019073486328125,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([35184372088832]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([35184372088832]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            -2.34844396355274555919e-22,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([79913407049891840]),
                    exp: -2,
                },
                sign: Sign::Neg,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([79913407049891840]),
                    exp: -2,
                },
                sign: Sign::Neg,
            },
        );
        test_conversion(
            1.04091361631528862002e-27,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([354204549120]),
                    exp: -2,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([354204549120]),
                    exp: -2,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            -1.83996007268899958108e+31,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([997444354048]),
                    exp: 1,
                },
                sign: Sign::Neg,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([997444354048]),
                    exp: 1,
                },
                sign: Sign::Neg,
            },
        );
        test_conversion(
            0.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );
        test_conversion(
            902341.2532,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4611686018427387904, 902341]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4670715600119529472, 902341]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            0239402.2340923,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4323455642275676160, 0239402]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4318240747745705984, 0239402]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            55.592082977294921875,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([10922003152559407104, 55]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([10922003152559407104, 55]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            1.95604696469614937424e-234,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([56019041081957608]),
                    exp: -13,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            6.2938409230490e102,
            CASNum {
                value: Infinite,
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([11491643866163970048, 2946572]),
                    exp: 5,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            0.5,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([9223372036854775808]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([9223372036854775808]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
        );

        test_conversion(
            1.04091361631528862002e-27,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([354204549120]),
                    exp: -2,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([354204549120]),
                    exp: -2,
                },
                sign: Sign::Pos,
            },
        );

        test_conversion(
            5.0653025104231136e-102,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([576460752303423488, 199583080902304]),
                    exp: -6,
                },
                sign: Sign::Pos,
            },
        );

        test_conversion(
            7.891780781773763e127,
            CASNum {
                value: Infinite,
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([13344165695898779648, 2002888061698]),
                    exp: 6,
                },
                sign: Sign::Pos,
            },
        );

        test_conversion(
            1.4186493757725223e-209,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([17822545243984101376, 1193]),
                    exp: -11,
                },
                sign: Sign::Pos,
            },
        );

        test_conversion(
            5.87962771339207829504e-23,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([20007336349270016]),
                    exp: -2,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([20007336349270016]),
                    exp: -2,
                },
                sign: Sign::Pos,
            },
        );

        test_conversion(
            56173.81640625,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([15060037153926938624, 56173]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([15060037153926938624, 56173]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );
    }

    #[test]
    fn addition_tests() {
        addition(1, 0);
        addition(0, 0);
        addition(0, 1);
        addition(-1, 0);
        addition(0, -1);
        addition(-1, -1);
        addition(1, -1);
        addition(-1, 1);
        addition(1, 1);

        addition(12032, 23420);
        addition(02312, 054123);
        addition(012312, 11231);
        addition(-52521, 01231);
        addition(10532153, -11252);
        addition(-235131, -65347641);
        addition(46589611, -15489456);
        addition(-541, 154);
        addition(154, 145);
    }
    #[test]
    fn subtraction_tests() {
        subtraction(1, 0);
        subtraction(0, 0);
        subtraction(0, 1);
        subtraction(-1, 0);
        subtraction(0, -1);
        subtraction(-1, -1);
        subtraction(1, -1);
        subtraction(-1, 1);
        subtraction(1, 1);

        subtraction(12032, 23420);
        subtraction(02312, 054123);

        subtraction(012312, 11231);
        subtraction(-52521, 01231);
        subtraction(10532153, -11252);
        subtraction(-235131, -65347641);
        subtraction(46589611, -15489456);
        subtraction(-541, 154);
        subtraction(154, 145);
    }

    #[test]
    fn comparison_tests() {
        comparison(1, 0);
        comparison(0, 0);
        comparison(0, 1);
        comparison(-1, 0);
        comparison(0, -1);
        comparison(-1, -1);
        comparison(1, -1);
        comparison(-1, 1);
        comparison(1, 1);

        comparison(12032, 23420);
        comparison(02312, 054123);
        comparison(012312, 11231);
        comparison(-52521, 01231);
        comparison(10532153, -11252);
        comparison(-235131, -65347641);
        comparison(46589611, -15489456);
        comparison(-541, 154);
        comparison(154, 145);
    }

    #[test]
    fn comparison_float_tests() {
        let floats_of_choice: Vec<f32> = vec![
            -2.34844396355274555919e-22,
            1.04091361631528862002e-27,
            -1.83996007268899958108e+31,
            0.,
            902341.2532,
            0239402.2340923,
            09.3423,
            -0.00304204920000,
        ];

        for a in &floats_of_choice {
            for b in &floats_of_choice {
                comparison_float(*a, *b);
            }
        }
    }
    #[test]
    fn addition_float_tests() {
        addition_float(
            0.,
            0.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        addition_float(
            0.,
            1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([1]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        addition_float(
            0.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([1]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );

        addition_float(
            1.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );

        addition_float(
            1.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([0]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );

        addition_float(
            100.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([99]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        addition_float(
            f64::INFINITY,
            f64::INFINITY,
            CASNum {
                value: Infinite,
                sign: Sign::Pos,
            },
        );

        addition_float(
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
            CASNum {
                value: Infinite,
                sign: Sign::Neg,
            },
        );

        addition_float(
            55.592082977294921875,
            13.384548187255859375,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([18015665146877181952, 68]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );
    }

    #[test]
    fn subtraction_float_tests() {
        subtraction_float(
            0.,
            0.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        subtraction_float(
            0.,
            1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([1]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );

        subtraction_float(
            0.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([1]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        subtraction_float(
            1.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([2]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        subtraction_float(
            100.,
            -1.,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([101]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        subtraction_float(
            55.592082977294921875,
            13.384548187255859375,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([3828341158241632256, 42]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        subtraction_float(
            13.384548187255859375,
            55.592082977294921875,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([3828341158241632256, 42]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );

        subtraction_float(
            -55.592082977294921875,
            13.384548187255859375,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([18015665146877181952, 68]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );

        subtraction_float(
            13.384548187255859375,
            -55.592082977294921875,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([18015665146877181952, 68]),
                    exp: 0,
                },
                sign: Sign::Pos,
            },
        );

        subtraction_float(
            5.87962771339207829504e-23,
            56173.81640625,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([18426736737360281600, 15060037153926938623, 56173]),
                    exp: 0,
                },
                sign: Sign::Neg,
            },
        );
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

        for value_1 in &values_to_test {
            for value_2 in &values_to_test {
                multiplication(*value_1, *value_2);
            }
        }
    }

    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;
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
            let reconstructed: f64 = cas_num.clone().into();
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
        let limit = 10;
        for _ in 0..limit {
            let float_1 = f64::from_bits(rng.next_u64());
            let float_2 = f64::from_bits(rng.next_u64());

            let rust_product = CASNum::from(float_1 * float_2);
            let my_product = CASNum::from(float_1) * &CASNum::from(float_2);

            if float_1.is_nan() || float_2.is_nan() {
                if !my_product.value.is_indeterminate() {
                    assert!(false);
                    num_wrong += 1;
                }
            }

            if rust_product != my_product {
                println!("{:e} {:e}", float_1, float_2);

                // assert_eq!(rust_product, my_product);
                num_wrong += 1;
            }
        }
        println!("{} / {}", limit - num_wrong, limit);
        assert_eq!(num_wrong, 0);
    }
}
