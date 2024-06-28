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

pub fn largest_divisible_subset_optimized(mut numbers: Vec<u32>) -> Vec<u32> {
    numbers.sort();

    let result = divisible_subset_optimized(&numbers[..], 1);
    return result[1..].to_vec()
}

fn divisible_subset_optimized(numbers: &[u32], current_factor: u32) -> Vec<u32> {
    let mut current_subset = Vec::new();
    for i in 0..numbers.len() {
        let alternative;

        if numbers[i] % current_factor == 0 {
            // BRANCH MITIGATION
            if let Some(index) = current_subset.iter().position(|&n| n == numbers[i]) {
                alternative = Vec::from(&current_subset[index..])
            }
            // BRANCHING
            else {

                alternative = divisible_subset_optimized(&numbers[i+1..], numbers[i]);
            }
            if alternative.len() > current_subset.len() {
                current_subset = alternative;
            }
        }
    }
    current_subset.insert(0, current_factor);
    return current_subset
}