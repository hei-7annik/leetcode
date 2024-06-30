use std::collections::BTreeMap;

// each factor f starts a subset S:
// each number N divisible by f_i creates a variant of that subset
// containing N as factor f_(i+1) where both subsets are equal for
// the first i elements
//
// the largest subset is obtained by BRANCHING for each N
pub fn largest_divisible_subset(mut numbers: Vec<u32>) -> Vec<u32> {
    numbers.sort();

    let result = divisible_subset(&numbers[..], 1);
    return result[1..].to_vec()
}

pub fn divisible_subset(numbers: &[u32], current_factor: u32) -> Vec<u32> {
    let mut current_subset = Vec::new();

    for i in 0..numbers.len() {
        if numbers[i] % current_factor == 0 {
            // BRANCHING
            let alternative= divisible_subset(&numbers[i+1..], numbers[i]);
            if alternative.len() > current_subset.len() {
                current_subset = alternative;
            }
        }
    }
    current_subset.insert(0, current_factor);
    return current_subset
}

pub fn largest_divisible_subset_optimized(numbers: Vec<u32>) -> Vec<u32> {
    let mut auto_complete = BTreeMap::new();
    return divisible_subset_optimized(&numbers, 1, &mut auto_complete);
}

fn divisible_subset_optimized(numbers: &Vec<u32>, current_factor: u32, auto_complete: &mut BTreeMap<u32, Vec<u32>>) -> Vec<u32> {
    let filtered_numbers: Vec<u32> = numbers.iter().filter_map(|&n| match n % current_factor {
        0 => Some(n),
        _ => None
    }).collect();

    let mut result = Vec::new();
    let mut current_longest_subset = Vec::new();

    for &factor in filtered_numbers.iter() {
        if factor == current_factor {
            result.push(current_factor);
        }
        else {
            let subset;
            if let Some(s) = auto_complete.get(&current_factor) {
                // BRANCH MITIGATION
                subset = s.clone();
            }
            else {
                // BRANCHING
                subset = divisible_subset_optimized(&filtered_numbers, factor, auto_complete);
            }

            if subset.len() > current_longest_subset.len() {
                current_longest_subset = subset;
            }
        }
    }
    auto_complete.insert(current_factor, current_longest_subset.clone());
    result.append(&mut current_longest_subset);

    return result
}

pub fn lds(mut numbers: Vec<u32>) -> Vec<u32> {
    numbers.sort();

    let mut size_subset_including = std::vec![1; numbers.len()];
    let mut i_subset_element_before = std::vec![numbers.len(); numbers.len()];

    let mut i_last_subset_element = 0;

    for current_i in 1..numbers.len() {
        for i in 0..current_i {
            if numbers[current_i] % numbers[i] == 0 {
                if size_subset_including[current_i] < size_subset_including[i] + 1 {
                    size_subset_including[current_i] = size_subset_including[i] + 1;
                    i_subset_element_before[current_i] = i;
                }

                if size_subset_including[current_i] > size_subset_including[i_last_subset_element] {
                    i_last_subset_element = current_i;
                }
            }
        }
    }
    let mut subset = Vec::with_capacity(size_subset_including[i_last_subset_element]);

    while let Some(&i) = i_subset_element_before.get(i_last_subset_element) {
        subset.push(numbers[i_last_subset_element]);
        i_last_subset_element = i;
    }
    // build answer
    subset.reverse();
    subset
}