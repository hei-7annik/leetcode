#[cfg(test)]
mod test {
    use leetcode::problem_368::largest_divisible_subset::largest_divisible_subset;

    mod with_unoptimized_solution {
        use super::*;

        #[test]
        fn with_continuous_numbers() {
            let numbers = Vec::from([1,2,3]);
            assert_eq!(largest_divisible_subset(numbers), [1,2]);
        }
        #[test]
        fn with_powers_of_two() {
            let numbers = Vec::from([1,2,4,8]);
            assert_eq!(largest_divisible_subset(numbers), [1,2,4,8]);
        }
        #[test]
        fn with_one_number() {
            let numbers = Vec::from([265]);
            assert_eq!(largest_divisible_subset(numbers), [265]);
        }
        #[test]
        fn with_even_numbers() {
            let numbers = Vec::from([1,2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,42,44,46,48,50,52,54,56,58,60,62,64,66,68,70,72,74,76,78,80,82,84,86,88,90,92,94,96,98,100,102,104,106,108,110,112,114]);
            assert_eq!(largest_divisible_subset(numbers), [1,2,4,8,16,32,64]);
        }
        #[test]
        fn with_odd_and_even_interleaved_numbers() {
            let numbers = Vec::from([1,2,4,3,9,27,108,81,144,540]);
            assert_eq!(largest_divisible_subset(numbers), [1,3,9,27,108,540]);
        }
    }
}
