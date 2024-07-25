#[cfg(test)]
mod problem_2385 {
    use std::rc::Rc;
    use std::cell::RefCell;
    use leetcode::problem_2385::calculate_infection_time::{
        calculate_infection_time,
        TreeNode
    };

    fn assemble_tree(values: &[u32], position: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&val) = values.get(position) {
            if val == 0 {
                return None
            }
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: assemble_tree(values, 2*position + 1),
                right: assemble_tree(values, 2*position + 2)}
            )))
        }
        None
    }

    mod naive_solution {
        use super::*;
        #[test]
        fn where_longest_path_includes_root() {
            let root = assemble_tree(&[1,5,3,0,4,10,6,0,0,9,2], 0);
            assert_eq!(calculate_infection_time(root, 3), 4)
        }

        #[test]
        fn where_longest_path_doesnt_include_root() {
            let root = assemble_tree(&[1,5,3,7,4,10,6,0,0,9,2,0,0,0,0,0,0,0,0,11], 0);
            assert_eq!(calculate_infection_time(root, 7), 4)
        }

        #[test]
        fn where_longest_path_is_from_infected_node_to_a_leaf() {
            let root = assemble_tree(&[1,5,3,0,4,10,6,0,0,9,2], 0);
            assert_eq!(calculate_infection_time(root, 1), 3)
        }

        #[test]
        fn where_longest_path_is_zero() {
            let root = assemble_tree(&[1], 0);
            assert_eq!(calculate_infection_time(root, 1), 0)
        }

        #[test]
        fn where_tree_is_a_linked_list() {
            let root = assemble_tree(&[1,0,2,0,0,0,3,0,0,0,0,0,0,0,4], 0);
            assert_eq!(calculate_infection_time(root, 1), 3)
        }

        #[test]
        fn where_tree_contains_only_copies_of_one_element() {
            let root = assemble_tree(&[1,0,1,0,0,0,1,0,0,0,0,0,0,0,4], 0);
            assert_eq!(calculate_infection_time(root, 4), 3)
        }
    }
}