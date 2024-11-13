#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::types::cas_num::Sign;

    use crate::types::cas_num::CASValue::Finite;
    use crate::types::cas_num::CASValue::Infinite;

    use super::super::CASNum;

    use rand::Rng;
    use rand::RngCore;

    fn comparison(a: i128, b: i128) {
        assert_eq!(CASNum::from(a) < CASNum::from(b), a < b);
        assert_eq!(CASNum::from(a) > CASNum::from(b), a > b);
        assert_eq!(CASNum::from(a) == CASNum::from(b), a == b);
    }

    fn addition(a: i128, b: i128) {
        let a_casnum = CASNum::from(a);
        let b_casnum = CASNum::from(b);
        let mut sum_1 = CASNum::from(a + b);
        let mut sum_2 = CASNum::from(a) + CASNum::from(b);
        sum_1.value = sum_1.value.normalize();
        sum_2.value = sum_2.value.normalize();
        assert_eq!(sum_1, sum_2);
    }

    fn subtraction(a: i128, b: i128) {
        let mut sum_1 = CASNum::from(a - b);
        let mut sum_2 = CASNum::from(a) - CASNum::from(b);
        sum_1.value = sum_1.value.normalize();
        sum_2.value = sum_2.value.normalize();
        assert_eq!(sum_1, sum_2);
    }

    fn multiplication(a: i128, b: i128) {
        let prod_1 = CASNum::from(a * b);
        let prod_2 = CASNum::from(a) * CASNum::from(b);
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
        let mut sum = CASNum::from(a) + CASNum::from(b);
        sum.value = sum.value.normalize();
        assert_eq!(sum, result);

        sum = CASNum::from(a as f32) + CASNum::from(b as f32);
        sum.value = sum.value.normalize();
        assert_eq!(sum, result);
    }

    fn subtraction_float(a: f64, b: f64, result: CASNum) {
        let mut sum = CASNum::from(a) - CASNum::from(b);
        sum.value = sum.value.normalize();
        assert_eq!(sum, result);

        sum = CASNum::from(a as f32) - CASNum::from(b as f32);
        sum.value = sum.value.normalize();
        assert_eq!(sum, result);
    }

    fn test_conversion(value: f64, desired_output_32: CASNum, desired_output_64: CASNum) {
        let thirty_two_bit = value as f32;
        let sixty_four_bit = value as f64;
        assert_eq!(CASNum::from(thirty_two_bit), desired_output_32);
        assert_eq!(CASNum::from(sixty_four_bit), desired_output_64);
    }

    // fn multiplication_float(a: f64, b: f64) {
    //     let prod_1 = CASNum::from(a * b);
    //     let prod_2 = CASNum::from(a) * CASNum::from(b);
    //     assert_eq!(prod_1, prod_2);
    // }

    // fn division_float(a: f64, b: f64) {
    //     let quot_1 = CASNum::from(a / b);
    //     let quot_2 = CASNum::from(a) / CASNum::from(b);
    //     assert_eq!(quot_1, quot_2);
    // }

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
                    exp: -1,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4670715600119529472, 902341]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            0239402.2340923,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4323455642275676160, 0239402]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4318240747745705984, 0239402]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            55.592082977294921875,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([10922003152559407104, 55]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
            CASNum {
                value: Finite {
                    digits: VecDeque::from([10922003152559407104, 55]),
                    exp: -1,
                },
                sign: Sign::Pos,
            },
        );
        test_conversion(
            1.95604696469614937424e-234,
            CASNum {
                value: Finite {
                    digits: VecDeque::from([]),
                    exp: -0,
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

        assert_eq!(
            CASNum::from(0239402.2340923),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([4318240747745705984, 0239402]),
                    exp: -1
                },
                sign: Sign::Pos,
            },
        );

        assert_eq!(
            CASNum::from(55.592082977294921875),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([10922003152559407104, 55]),
                    exp: -1
                },
                sign: Sign::Pos,
            },
        );

        assert_eq!(
            CASNum::from(1.95604696469614937424e-234),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([56019041081957608]),
                    exp: -13
                },
                sign: Sign::Pos,
            },
        );

        assert_eq!(
            CASNum::from(6.2938409230490e102),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([11491643866163970048, 2946572]),
                    exp: 5
                },
                sign: Sign::Pos,
            },
        );

        assert_eq!(
            CASNum::from(0.5),
            CASNum {
                value: Finite {
                    digits: VecDeque::from([9223372036854775808]),
                    exp: -1
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
                    exp: -1,
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
                    exp: -1,
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
                    exp: -1,
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
                    exp: -1,
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
                    exp: -1,
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
                    exp: -2,
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
}
