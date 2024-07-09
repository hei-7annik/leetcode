mod problem_1823 {
    use leetcode::problem_1823::find_the_winner::{find_the_winner, find_the_winner_optimized};

    mod naive_solution {
        use super::*;
        #[test]
        fn with_5_friends_and_2_as_integer() {
            assert_eq!(find_the_winner(5,3), 3)
        }

        #[test]
        fn with_6_friends_and_5_as_integer() {
            assert_eq!(find_the_winner(6,5), 1)
        }

        #[test]
        fn with_3_friends_and_1_as_integer() {
            assert_eq!(find_the_winner(3,1), 3)
        }
    }

    mod optimized_solution {
        use super::*;
        #[test]
        fn with_5_friends_and_2_as_integer() {
            assert_eq!(find_the_winner_optimized(5,2), 3)
        }

        #[test]
        fn with_6_friends_and_5_as_integer() {
            assert_eq!(find_the_winner_optimized(6,5), 1)
        }
    }
}