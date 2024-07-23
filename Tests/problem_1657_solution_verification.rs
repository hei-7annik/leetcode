#[cfg(test)]
mod problem_1657 {
    use leetcode::problem_1657::close_strings::{close_strings};

    mod solution {
        use super::*;

        #[test]
        fn with_words_having_characters_in_different_order() {
            assert!(close_strings(String::from("abc"), String::from("bca")))
        }

        #[test]
        fn with_words_containing_a_single_characters() {
            assert!(close_strings(String::from("a"), String::from("a")))
        }

        #[test]
        fn with_words_having_a_different_character_distribution() {
            assert!(close_strings(String::from("cabbba"), String::from("abbccc")))
        }

        #[test]
        fn with_words_having_distinct_character_sets() {
            assert_eq!(close_strings(String::from("uiuiiuuiuuuuuuwiwuuwiiiiuuuuwwiwuuu"),
                                  String::from("rppprrpurrrrrurrrurprprprprpuprruur")), false)
        }
    }
}
