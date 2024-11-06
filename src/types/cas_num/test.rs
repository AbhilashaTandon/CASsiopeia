#[cfg(test)]
pub mod test {
    use super::super::CASNum;

    #[test]
    fn comparisons() {
        let a: CASNum = *CASNum::new(12);
        let b: CASNum = *CASNum::new(12343242);
        assert!(a < b);
        let c: CASNum = *CASNum::new(12);
        assert!(a == c);
        assert!(b > a);

        let d = *CASNum::new(-256000);
        assert!(d < a);
        assert!(b > d);
        assert!(c > d);
    }

    fn addition(a: i128, b: i128) {
        let mut sum_1 = *CASNum::new(a + b);
        let mut sum_2 = *CASNum::new(a) + *CASNum::new(b);
        assert_eq!(sum_1.normalize(), sum_2.normalize());
    }

    fn subtraction(a: i128, b: i128) {
        let mut sum_1 = *CASNum::new(a - b);
        let mut sum_2 = *CASNum::new(a) - *CASNum::new(b);
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
}
