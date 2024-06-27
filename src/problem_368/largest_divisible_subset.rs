// multiply all numbers in the solution + 1 additional a factor

pub fn largest_divisible_subset(numbers: &[u32]) -> Vec<u32> {
    let mut current_subset = Vec::new();
    let current_factor = numbers[0];

    for i in 1..numbers.len() {

        if numbers[i] % current_factor == 0 {
            let alternative = largest_divisible_subset(&numbers[i..]);
            if alternative.len() > current_subset.len() {
                current_subset = alternative;
            }
        }
    }
    current_subset.insert(0, current_factor);
    return current_subset
}