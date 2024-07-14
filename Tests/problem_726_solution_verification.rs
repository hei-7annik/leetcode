#[cfg(test)]
mod problem_726 {
    use leetcode::problem_726::count_of_atoms::{count_of_atoms/*, count_of_atoms_optimized*/};
    mod unoptimized_solution {
        use super::*;

        #[test]
        fn with_() {
            let formula = "".to_string();
            assert_eq!(count_of_atoms(formula), "")
        }
    }

    mod partially_optimized_solution {
        use super::*;

        #[test]
        fn with_() {
            let formula = "".to_string();
            assert_eq!(count_of_atoms(formula), "")
        }
    }
}