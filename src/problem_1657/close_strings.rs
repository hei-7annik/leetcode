use std::collections::{BTreeMap};

fn get_char_set_and_char_distribution(word: String) -> BTreeMap<char, u32> {
    let mut characters: BTreeMap<char, u32> = BTreeMap::new();
    word.chars().for_each(|c| {
        characters.entry(c).and_modify( | occurences| *occurences += 1 ).or_insert(1);
    });

    characters
}

/// Checks if one string can be converted into another by repeatedly applying either
/// (**Operation 1**) swapping two characters or (**Operation 2**) exchanging all occurrences of one
/// kind of character with that of another.
///
/// # Example
/// ```rust
/// assert_eq!(close_strings("cabbba".to_string(), "abbccc".to_string()),true)
/// ```
///
/// # Cases
/// 1. Word `1` & `2` are the same
/// 2. Word `1` & `2` have the same set of characters and the same distribution
/// 3. Word `1` & `2` dont have the same set of character or distribution
///
/// # Method
/// 1. Get character set and distribution `"cabba"` -> `{(a: 2),(b: 3), (c: 1)}`
/// 2. Use **Operation 2** to change how often a character occurs to align their character distribution
/// 3. Use **Operation 1** to change the characters position to align their order
///
pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        false
    }
    else {
        let features_word1 = get_char_set_and_char_distribution(word1);
        let features_word2 = get_char_set_and_char_distribution(word2);

        let chars_subset_equal = features_word1.keys().all(|key| features_word2.contains_key(key));
        let mut chars_distribution_w1: Vec<u32> = features_word1.into_values().collect();
        chars_distribution_w1.sort();

        let mut chars_distribution_w2: Vec<u32> = features_word2.into_values().collect();
        chars_distribution_w2.sort();

        chars_subset_equal &&
            chars_distribution_w1 == chars_distribution_w2
    }
}