use std::collections::BTreeMap;
///
/// To-Do Add documentation to each function
///
pub fn count_of_atoms(formula: String) -> String {
    let mut factors = vec![1];
    let mut factor_len = 0;
    let mut symbol_len = 0;

    let mut element_count: BTreeMap<&str, u32> = BTreeMap::new();

    formula.chars().rev().enumerate().for_each(|(index, character)| {
        let i = formula.len() - 1 - index;
        match character {
            C if C.is_ascii_uppercase() => {
                let periodic_table_element_symbol = &formula[i..=i + symbol_len];
                let count_string = &formula[i + symbol_len + 1..=i + symbol_len + factor_len];
                let count = count_string.parse().unwrap_or(1) * factors.last().unwrap();

                element_count
                    .entry(periodic_table_element_symbol)
                    .and_modify(|old_count| *old_count += count)
                    .or_insert(count);

                factor_len = 0;
                symbol_len = 0;
            },
            c if c.is_ascii_lowercase() =>
                symbol_len += 1,
            number if number.is_digit(10) => {
                factor_len += 1;
            },
            '(' => {
                factors.pop(); ()
            },
            ')' => {
                let factor = &formula[i + 1..=i + factor_len];
                factors.push(factor.parse().unwrap_or(1) * factors.last().unwrap());
                factor_len = 0;
            },
            _ => (),
        }
    });

    element_count
        .iter()
        .fold(String::with_capacity(element_count.len() * 3),|mut result, (&symbol, &count)| {
            result.push_str(symbol);
            if count != 1 {
                result.push_str(&count.to_string())
            };
            result
        })
}