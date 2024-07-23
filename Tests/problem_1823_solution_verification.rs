mod problem_1823 {
    use leetcode::problem_1823::find_the_winner::{
        find_the_winner,
        find_the_winner_optimized
    };

    mod naive_solution {
        use super::*;
        #[test]
        fn with_small_group_of_friends() {
            assert_eq!(find_the_winner(5,2), 3)
        }

        #[test]
        fn with_larger_group_of_friends() {
            assert_eq!(find_the_winner(6,5), 1)
        }

        #[test]
        fn with_step_size_1() {
            assert_eq!(find_the_winner(3,1), 3)
        }

        #[test]
        fn with_single_person() {
            assert_eq!(find_the_winner(1,2), 1)
        }
    }

    mod optimized_solution {
        use super::*;
        #[test]
        fn with_small_group_of_friends() {
            assert_eq!(find_the_winner_optimized(5,2), 3)
        }

        #[test]
        fn with_larger_group_of_friends() {
            assert_eq!(find_the_winner_optimized(6,5), 1)
        }

        #[test]
        fn with_step_size_1() {
            assert_eq!(find_the_winner(3,1), 3)
        }

        #[test]
        fn with_single_person() {
            assert_eq!(find_the_winner(1,2), 1)
        }
    }
}