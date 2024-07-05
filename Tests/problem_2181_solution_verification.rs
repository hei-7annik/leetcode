mod test {
    use leetcode::problem_2181::merge_nodes::{merge_nodes, list_from};

    mod naive_solution {
        use super::*;
        #[test]
        fn with_list_containing_two_sections() {
            let list = list_from(&[0,3,1,0,4,5,2,0]);
            let result = list_from(&[4,11]);
            assert_eq!(merge_nodes(list), result)
        }

        #[test]
        fn with_list_containing_three_sections() {
            let list = list_from(&[0,1,0,3,0,2,2,0]);
            let result = list_from(&[1,3,4]);
            assert_eq!(merge_nodes(list), result)
        }
    }
}