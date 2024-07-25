#[cfg(test)]
mod problem_7 {
    use leetcode::problem_7::reverse::{reverse};
    mod naive_solution {
        use super::*;
        #[test]
        fn with_positive_number() {
            let number = 123;
            assert_eq!(reverse(number), 321);
        }

        #[test]
        fn with_negative_number() {
            let number = -123;
            assert_eq!(reverse(number), -321);
        }

        #[test]
        fn with_trailing_zeros() {
            let number = 120;
            assert_eq!(reverse(number), 21);
        }

        #[test]
        fn with_leading_zeros() {
            let number = 00120;
            assert_eq!(reverse(number), 21);
        }

        #[test]
        fn with_single_digit() {
            let number = 0;
            assert_eq!(reverse(number), 0);
        }

        #[test]
        fn with_max_i32_number() {
            let number = 2147483647;
            assert_eq!(reverse(number), 0);
        }

        #[test]
        fn with_min_i32_number() {
            let number = -2147483648;
            assert_eq!(reverse(number), 0);
        }
    }
}