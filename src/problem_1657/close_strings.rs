use std::collections::{BTreeMap};

fn get_char_set_and_char_distribution(word: String) -> BTreeMap<char, u32> {
    let mut characters: BTreeMap<char, u32> = BTreeMap::new();
    word.chars().for_each(|c| {
        characters.entry(c).and_modify( | occurences| *occurences += 1 ).or_insert(1);
    });

    characters
}

// words with the same set of characters and the same distribution
//
// (char x_1: 6 times, char x_2: 3 times... )
//
// can be transformed into each other using operation 1&2:
//
// Operation 1 allows for changing where characters occur
// Operation 2 allows for changing how often characters occur
// if and only if there is another (kind of) character to swap

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        false
    }
    else {
        let features_word1 = get_char_set_and_char_distribution(word1);
        let features_word2 = get_char_set_and_char_distribution(word2);

        let chars_subset_equal = features_word1.keys().any(|key| features_word2.contains_key(key));
        let mut chars_distribution_w1: Vec<u32> = features_word1.into_values().collect();
        chars_distribution_w1.sort();

        let mut chars_distribution_w2: Vec<u32> = features_word2.into_values().collect();
        chars_distribution_w2.sort();

        chars_subset_equal &&
            chars_distribution_w1 == chars_distribution_w2
    }
}