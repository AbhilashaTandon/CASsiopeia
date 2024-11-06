#[cfg(test)]
pub mod test {
    use super::super::CASNum;

    #[test]
    fn comparisons() {
        let a: CASNum = *CASNum::new(12);
        let mut b: CASNum = *CASNum::new(12343242);
        b.normalize();
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
        let sum_1 = *CASNum::new(a + b);
        let sum_2 = *CASNum::new(a) + *CASNum::new(b);
        assert_eq!(sum_1, sum_2);
    }

    #[test]
    fn addition_tests() {
        addition(1, 0);
        addition(0, 0);
        addition(0, 1);
        addition(-1, 0);
        addition(0, -1);
    }
}
