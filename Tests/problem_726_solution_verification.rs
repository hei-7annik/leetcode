#[cfg(test)]
mod problem_726 {
    use leetcode::problem_726::count_of_atoms::{count_of_atoms/*, count_of_atoms_optimized*/};
    mod solution {
        use super::*;

        #[test]
        fn with_simple_formula() {
            let formula = "H2O".to_string();
            assert_eq!(count_of_atoms(formula), "H2O")
        }
        #[test]
        fn with_one_parenthesis() {
            let formula = "Mg(OH)2".to_string();
            assert_eq!(count_of_atoms(formula), "H2MgO2")
        }
        #[test]
        fn with_nested_parenthesis() {
            let formula = "K4(ON(SO3)2)2".to_string();
            assert_eq!(count_of_atoms(formula), "K4N2O14S4")
        }

        #[test]
        fn with_a_lot_of_atoms() {
            let formula = "Be32".to_string();
            assert_eq!(count_of_atoms(formula), "Be32")
        }
    }
}