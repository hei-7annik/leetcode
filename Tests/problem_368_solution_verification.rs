#[cfg(test)]
mod test {
    use leetcode::problem_368::largest_divisible_subset::largest_divisible_subset;

    mod with_naive_solution {
        use super::*;

        #[test]
        fn where_the_numbers_steadily_increase() {
            let numbers = Vec::from([1,2,3]);
            assert_eq!(largest_divisible_subset(numbers), [1,2]);
        }
        #[test]
        fn where_the_numbers_are_powers_of_two() {
            let numbers = Vec::from([1,2,4,8]);
            assert_eq!(largest_divisible_subset(numbers), [1,2,4,8]);
        }
    }

    mod with_optimized_solution {
        // use super::*;

    }
}
