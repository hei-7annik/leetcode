#[cfg(test)]
mod problem_135 {
    use leetcode::problem_135::candy::{candy};
    mod solution {
        use super::*;
        #[test]
        fn with_all_different_ratings() {
            let ratings = vec![1,0,2];
            assert_eq!(candy(ratings),5);
        }

        #[test]
        fn with_two_equal_ratings_next_to_each_other() {
            let ratings = vec![1,2,2];
            assert_eq!(candy(ratings),4);
        }
    }
}