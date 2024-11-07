#[cfg(test)]
pub mod test {
    use super::super::CASNum;

    fn comparison(a: i128, b: i128) {
        assert_eq!(CASNum::new(a) < CASNum::new(b), a < b);
        assert_eq!(CASNum::new(a) > CASNum::new(b), a > b);
        assert_eq!(CASNum::new(a) == CASNum::new(b), a == b);
    }

    fn addition(a: i128, b: i128) {
        let mut sum_1 = CASNum::new(a + b);
        let mut sum_2 = CASNum::new(a) + CASNum::new(b);
        assert_eq!(sum_1.normalize(), sum_2.normalize());
    }

    fn subtraction(a: i128, b: i128) {
        let mut sum_1 = CASNum::new(a - b);
        let mut sum_2 = CASNum::new(a) - CASNum::new(b);
        assert_eq!(sum_1.normalize(), sum_2.normalize());
    }

    fn comparison_float(a: f32, b: f32) {
        assert_eq!(CASNum::new(a) < CASNum::new(b), a < b);
        assert_eq!(CASNum::new(a) > CASNum::new(b), a > b);
        assert_eq!(CASNum::new(a) == CASNum::new(b), a == b);
    }

    fn addition_float(a: f32, b: f32) {
        let mut sum_1 = CASNum::new(a + b);
        let mut sum_2 = CASNum::new(a) + CASNum::new(b);
        assert_eq!(sum_1.normalize(), sum_2.normalize());
    }

    fn subtraction_float(a: f32, b: f32) {
        let mut sum_1 = CASNum::new(a - b);
        let mut sum_2 = CASNum::new(a) - CASNum::new(b);
        assert_eq!(sum_1.normalize(), sum_2.normalize());
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
                addition_float(*a, *b);
            }
        }
    }

    #[test]
    fn subtraction_float_tests() {
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
                subtraction_float(*a, *b);
            }
        }
    }
}
