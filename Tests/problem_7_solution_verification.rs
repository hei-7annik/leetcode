#[cfg(test)]
mod problem_7 {
    use leetcode::problem_7::reverse::{reverse};
    mod naive_solution {
        use super::*;
        #[test]
        fn with_basic_number() {
            let number = 123;
            assert_eq!(reverse(number), 321);
        }

        fn with_negative_number() {
            let number = -123;
            assert_eq!(reverse(number), -321);
        }

        fn with_last_digit_is_zero() {
            let number = 120;
            assert_eq!(reverse(number), 21);
        }
    }
}