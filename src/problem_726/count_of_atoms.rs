use std::collections::BTreeMap;
pub fn count_of_atoms(formula: String) -> String {
    let mut multiplier = Vec::from([1]);
    let mut occurence = String::new();

    let mut element_distribution: BTreeMap<String, u32> = BTreeMap::new();
    let mut periodic_table_element_symbol = String::with_capacity(2);

    formula.chars().rev().for_each(|char|
        match char {
            C if C.is_ascii_uppercase() => {
                periodic_table_element_symbol.insert(0, C);
                if let Some(old_occurrences) = element_distribution.insert(
                    periodic_table_element_symbol.clone(),
                    occurence.parse::<u32>().unwrap_or(1) * multiplier.last().unwrap()) {
                        element_distribution
                            .entry(periodic_table_element_symbol.clone())
                            .and_modify(|occurences| *occurences += old_occurrences);
                };
                
                periodic_table_element_symbol.clear();
                occurence.clear()},
            c if c.is_ascii_lowercase() => {
                periodic_table_element_symbol.push(c)},
            number if number.is_digit(10) =>
                occurence.insert(0, number),
            '(' => {
                multiplier.pop(); ()},
            ')' => {
                multiplier.push(occurence.parse::<u32>().unwrap_or(1) * multiplier.last().unwrap());
                occurence.clear()},
            _ => (),
        }
    );
    let mut result = String::new();
    element_distribution.iter()
        .for_each(|(key, value)| {
            result.push_str(&key);
            if *value > 1 {
                result.push_str(&value.to_string());
            }
        });

    result
}