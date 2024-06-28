#[cfg(test)]
mod test {
    use leetcode::problem_1657::close_strings::{close_strings};

    mod naive_solution {
        use super::*;

        #[test]
        fn with_word1_abc_and_word2_bca() {
            assert!(close_strings(String::from("abc"), String::from("bca")))
        }

        #[test]
        fn with_word1_a_and_word2_aa() {
            assert!(close_strings(String::from("a"), String::from("a")))
        }

        #[test]
        fn with_word1_cabbba_and_word2_abbccc() {
            assert!(close_strings(String::from("cabbba"), String::from("abbccc")))
        }
    }

    mod optimized_solution {
        // use super::*;
    }
}
