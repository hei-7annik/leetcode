#[cfg(test)]
mod count_triplets {
    use leetcode::problem_1442::count_triplets::{
    count_triplets_optimized,
    count_triplets_naive };

    // input constraints
    static MAX_VECTOR_SIZE: usize = 300;
    static MAX_INT_SIZE: u32 = 100000000;

    fn verify_input(vector: Vec<u32>) -> Vec<u32>{
        assert!(vector.len().le(&MAX_VECTOR_SIZE));
        assert_eq!(vector.iter().find(|&x| { x > &MAX_INT_SIZE}),None);

        vector
    }

    // test that check each solution for compliance with i < j <= k?

    mod with_naive_solution {
        use super::*;

        #[test]
        fn in_random_input() {
            let vector: Vec<u32> = verify_input(Vec::from([2,3,1,6,7]));

            assert_eq!(count_triplets_naive(&vector), 4);
        }

        #[test]
        fn in_same_number_input() {
            let vector: Vec<u32> = verify_input(Vec::from([1,1,1,1,1]));

            assert_eq!(count_triplets_naive(&vector), 10);
        }
    }

    mod with_optimized_solution {
        use super::*;

        #[test]
        fn in_random_input() {
            let vector: Vec<u32> = verify_input(Vec::from([2,3,1,6,7]));

            assert_eq!(count_triplets_optimized(&vector), 4);
        }

        #[test]
        fn in_same_number_input() {
            let vector: Vec<u32> = verify_input(Vec::from([1,1,1,1,1]));

            assert_eq!(count_triplets_optimized(&vector), 10);
        }
    }
}