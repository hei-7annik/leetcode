// multiply all numbers in the solution + 1 additional a factor

pub fn largest_divisible_subset(numbers: &[u32]) -> Vec<u32> {
    let mut subset = Vec::new();

    for index in 1..numbers.len() {
        let num = numbers[index];
        if num % numbers[0] == 0 {

            let addition_factors = largest_divisible_subset(&numbers[index..]);
            if addition_factors.len() > subset.len() {
                subset = addition_factors;
            }
        }
    }
    subset.insert(0, numbers[0]);
    return subset
}