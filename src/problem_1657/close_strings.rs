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
        let char_set_w1: Vec<char> = features_word1.keys().cloned().collect();
        let mut char_distribution_w1: Vec<u32> = features_word1.values().cloned().collect();
        char_distribution_w1.sort();

        let features_word2 = get_char_set_and_char_distribution(word2);
        let char_set_w2: Vec<char> = features_word2.keys().cloned().collect();
        let mut char_distribution_w2: Vec<u32> = features_word2.values().cloned().collect();
        char_distribution_w2.sort();

        char_set_w1 == char_set_w2 &&
            char_distribution_w1 == char_distribution_w2
    }
}