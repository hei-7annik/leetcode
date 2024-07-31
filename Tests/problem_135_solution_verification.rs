#[cfg(test)]
mod problem_135 {
    use leetcode::problem_135::candy::{candy, candy_optimized};
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

        #[test]
        fn with_one_rating() {
            let ratings = vec![1];
            assert_eq!(candy(ratings),1);
        }

        #[test]
        fn with_increasing_decreasing_and_equal_ratings() {
            let ratings = vec![1,2,3,6,4,2,3,4,0,0];
            assert_eq!(candy(ratings),20);
        }

        #[test]
        fn with_increasing_and_decreasing_ratings() {
            let ratings = vec![1,2,3,6,4,2,3,4];
            assert_eq!(candy(ratings),18);
        }

        #[test]
        fn with_equal_increasing_and_decreasing_ratings_with_peak() {
            let ratings = vec![1,2,3,6,4,3,2,1];
            assert_eq!(candy(ratings),21);
        }

        #[test]
        fn with_equal_increasing_and_decreasing_ratings_without_peak() {
            let ratings = vec![2,3,6,6,3,2];
            assert_eq!(candy(ratings),12);
        }

        #[test]
        fn with_more_decreasing_than_increasing_ratings_with_peak() {
            let ratings = vec![2,3,8,6,3,2,1];
            assert_eq!(candy(ratings),18);
        }

        #[test]
        fn with_increasing_decreasing_ratings() {
            let ratings = vec![1,0,2,3,4];
            assert_eq!(candy(ratings),12);
        }

        #[test]
        fn with_decreasing_ratings() {
            let ratings = vec![4,3,2,1,0];
            assert_eq!(candy(ratings),15);
        }

        #[test]
        fn with_increasing_ratings() {
            let ratings = vec![1,2,3,4,5];
            assert_eq!(candy(ratings),15);
        }

        #[test]
        fn with_equal_ratings() {
            let ratings = vec![4,4,4,4,4];
            assert_eq!(candy(ratings),5);
        }

        #[test]
        fn with_decreasing_and_equal_ratings() {
            let ratings = vec![4,3,2,2,2,1,0];
            assert_eq!(candy(ratings),13);
        }

        #[test]
        fn with_two_ratings() {
            let ratings = vec![3,4];
            assert_eq!(candy(ratings),3);
        }

    }
}