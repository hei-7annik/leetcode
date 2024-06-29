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
    let mut indices = std::vec![numbers.len(); numbers.len()];
    let mut sizes = std::vec![1; numbers.len()];

    let mut last_index = 0;

    for index in 1..numbers.len() {
        let current_number = numbers[index];

        numbers[..index].iter().enumerate().for_each(|(i, &factor)| {
            if current_number % factor == 0 {

                if sizes[index] < sizes[i] + 1 {
                    sizes[index] = sizes[i] + 1;
                    indices[index] = i;
                }

                if sizes[index] > sizes[last_index] {
                    last_index = index;
                }
            }
        })
    }
    let mut subset = Vec::with_capacity(sizes[last_index]);

    while let Some(&i) = indices.get(last_index) {
        subset.push(numbers[last_index]);
        last_index = i;
    }
    // build answer
    subset.reverse();
    subset
}