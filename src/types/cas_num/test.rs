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

    #[test]
    fn conversion_tests_float_2() {
        let float_bits: Vec<u64> = vec![
            0x2b08f2308ecba2b8,
            0x96d597480ab29240,
            0x9b2f50d91954289c,
            0xfff14049d2dd4ea5,
            0x3f1a098a67b08c04,
            0x53115f71d3e074c0,
            0x3717f73419b72ef4,
            0xbb18023a9a734f84,
            0x9f02e1d9a6f59d60,
            0x9b27a6d6184a3e66,
            0xab226e9d6772dc8a,
            0xfff63b7323a068b1,
            0x5b2f5f320d61d3c2,
            0x531dcfbbed809408,
            0xc700be655f6f56a8,
            0xcb28efcc9e8fe16e,
            0xd6d20c403c15f680,
            0xa719c6aa1cbe94b8,
            0x8b2e7448de249208,
            0xc31277f5fcf10288,
            0xcf1aef1539071b14,
            0x6f2914345f175af8,
            0x7ee243001def45c0,
            0xc329431103eb9900,
            0xe32274ed14bd13fa,
            0xfb28a7c54f13c4f8,
            0x3b2f2caf9144cd2e,
            0xff142d6868c1c4d8,
            0xab0844a7dcbe9210,
            0x8f293f512e5c6eac,
            0xeaf4027bf1b2e090,
            0xfb109da99ce7ae48,
            0x7f2803a56c2b6ffe,
            0xf32472e2dcf2f1be,
            0xcf132c601b6f82a8,
            0x3164f4fb59fa8dc,
            0xfff6a95d80910e2e,
            0x8f2102f6897e89ec,
            0x832eb7f09f6128c6,
            0x6f226188876adc5c,
            0xc727228ff69e4504,
            0x328f3df78bf8234,
            0xfeef95d64d9f4560,
            0x4f234b47ceda6b54,
            0xfb1bd060a5dadb04,
            0x3b267328470694e8,
            0xf2f712b8cfadee50,
            0x3b2125e787f1a1aa,
            0x73134f09632a0078,
            0xe72d334aba3a457e,
            0xa323ecac05bb9282,
            0x3dd9ce0834800000,
            0xdb2d51fc0bb15eaa,
            0x3f1a76fa13d05718,
            0xe71ed0bf3a433e58,
            0x2f23e5537bccb428,
            0x72579ee99ac2c40,
            0xff2832aa9afe0bfc,
            0x9b26531deed8ac2c,
            0x23174b0737b6e1a4,
            0xa71d2169e7c9509c,
            0x6ae8209316d4a0a0,
            0x7ffa73c120b330c0,
            0xe727a56412cf5fda,
            0x7f00209932b142f0,
            0x1726f2a519ed71a4,
            0x172ca02c725e4988,
            0x1f1a4179685d418c,
            0xa724a4e1f6ef3c04,
            0x1317fd6aa93832b0,
            0x2f2d2463faa2bb08,
            0xd72904718435e97c,
            0x2b1e771bf46c7730,
            0xf3201aeab18e652c,
            0x4af0992a766f7d10,
            0x4f1cd4ec8162e9d4,
            0x3f2e85b092475840,
            0x17175c30e944c65c,
            0x772419329e718140,
            0x6709b5cc7a9f19c8,
            0xdb29eaed0ea339c6,
            0xb32506106d6b0530,
            0x272ed4bd15cc4bf2,
            0x772b6bbec9c3c07a,
            0xdee289a5928ece20,
            0xb729ff1a538e1c48,
            0x7f219f7c3b123250,
            0x230ac05f2a346d30,
            0xfb2be5f50357a246,
            0xcf15356dac8a99e8,
            0xaafd767a1c33e9f0,
            0x332f1a667b17fcf4,
            0x4321f51e12616e00,
            0x7f1eb98839b0a138,
            0x3b2d4c4a9a6e6ffa,
            0xaeca7e2fabd0da0,
            0xfabdd52ff4f83800,
            0x472b3a732875c99a,
            0x3701b4f2a87f4878,
            0xffff559b6ac42123,
            0xb2cc381737a7f100,
            0x4f200320072d0b0e,
            0x1f2520a3b105940a,
            0x272c19e30fa4fa38,
            0xa72088677d0459c8,
            0xb2eb8fa514122280,
            0xb71b94eb18ea2500,
            0x930f896294a92330,
            0xc32ff989e7f72c26,
            0xdb291f26f3b31400,
            0xd3036a1a882bf490,
            0xab15a5d471be6584,
            0x6715ccc605c83894,
            0xfb2b392f31f5e874,
            0xfee8ebac984a40e0,
            0xbf229356211216d6,
            0x6321b3a803871bfc,
            0x2727601325871ed4,
            0xe6ebbf1a6a0519c0,
            0x5324f16acba39dba,
            0x431600cc8c39ade4,
            0x7ff28511aedf4a61,
            0xcf155db30032f818,
            0x9706a0335d0bf0e8,
            0x3706a90a77d744f8,
            0x2b1d8f1a130b28fc,
            0x8325b975e9d84108,
            0x3f221c9118216844,
            0xfef4a33a90fb9390,
            0x33294709ca1ba55e,
            0x6729c5db0bb46780,
            0xcb12fbdd830a96dc,
            0x370649a8e73459c0,
            0xaf2097642acf9df4,
            0xcb225b5068386480,
            0xeb1a32de319cbf74,
            0x12ebe8715a8b9fc0,
            0xcf2f3a204ecd49e0,
            0x5b2250a757a45a66,
            0xffff80a4179b5370,
            0xb6cc230eaf4a5700,
            0x172ea7b065109c28,
            0xcf1178f62f3304ac,
            0x3f2de03ad54a94ea,
            0xc3233700e5737ef2,
            0xcf2fed65e5ac269a,
            0xb21a91d02af0a34,
            0x172e9c56184c5172,
            0x9f2dc596d2d45bda,
            0xcf2217b3d3f6742a,
            0xab078a32249414c8,
            0xc6f1ed610a603210,
            0x70afc3767679f00,
            0x572618ea434f10b0,
            0xeb23140c1bf479e6,
            0xcadf452f04647b40,
            0xfb21da31d18d19ec,
        ];

        for bits in float_bits {
            let float: f64 = f64::from_bits(bits);
            let cas_num = CASNum::from(float);
            let reconstructed: f64 = cas_num.clone().into();
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
}
