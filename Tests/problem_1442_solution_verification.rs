#[cfg(test)]
mod problem_1442 {
    use leetcode::problem_1442::count_triplets::{
        count_triplets_optimized,
        count_triplets
    };

    // input constraints
    static MAX_VECTOR_SIZE: usize = 300;
    static MAX_INT_SIZE: u32 = 100000000;

    fn verify(input: Vec<u32>) -> Vec<u32> {
        assert!(input.len().le(&MAX_VECTOR_SIZE));
        assert_eq!(input.iter().find(|&x| { x > &MAX_INT_SIZE}),None);

        input
    }

    mod solution {
        use super::*;

        #[test]
        fn in_random_input() {
            let vector: Vec<u32> = verify(vec![2,3,1,6,7]);

            assert_eq!(count_triplets(&vector), 4);
        }

        #[test]
        fn in_same_number_input() {
            let vector: Vec<u32> = verify(vec![1,1,1,1,1]);

            assert_eq!(count_triplets(&vector), 10);
        }
    }

    mod optimized_solution {
        use super::*;

        #[test]
        fn in_random_input() {
            let vector: Vec<u32> = verify(vec![2,3,1,6,7]);

            assert_eq!(count_triplets_optimized(&vector), 4);
        }

        #[test]
        fn in_same_number_input() {
            let vector: Vec<u32> = verify(vec![1,1,1,1,1]);

            assert_eq!(count_triplets_optimized(&vector), 10);
        }
    }
}