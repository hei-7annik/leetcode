// each factor f starts a subset S:
// each number N divisible by f_i creates a variant of that subset
// containing N as factor f_(i+1) where both subsets are equal for
// the first i elements
//
// the largest subset is obtained by BRANCHING for each N
pub fn largest_divisible_subset(numbers: &[u32]) -> Vec<u32> {
    let mut current_subset = Vec::new();
    let current_factor = numbers[0];

    for i in 1..numbers.len() {

        if numbers[i] % current_factor == 0 {
            // BRANCHING
            let alternative = largest_divisible_subset(&numbers[i..]);
            if alternative.len() > current_subset.len() {
                current_subset = alternative;
            }
        }
    }
    current_subset.insert(0, current_factor);
    return current_subset
}

pub fn largest_divisible_subset_optimized(numbers: &[u32]) -> Vec<u32> {
    let mut current_subset = Vec::new();
    let current_factor = numbers[0];

    for i in 1..numbers.len() {

        let alternative;

        let num = numbers[i];
        if num % current_factor == 0 {
            // BRANCH MITIGATION
            if let Some(index) = current_subset.iter().position(|&n| n == num) {
                alternative = Vec::from(&current_subset[index..])
            }
            // BRANCHING
            else {
                alternative = largest_divisible_subset(&numbers[i..]);
            }
            if alternative.len() > current_subset.len() {
                current_subset = alternative;
            }
        }
    }
    current_subset.insert(0, current_factor);
    return current_subset
}