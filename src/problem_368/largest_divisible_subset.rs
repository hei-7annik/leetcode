// multiply all numbers in the solution + 1 additional a factor

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    nums.reverse();

    let mut answer = Vec::new();

    while let Some(new_factor) = nums.pop() {
        // ToDo: fix impact of ordering in input numbers -> least limiting choice
        if answer.iter().all(|factor| new_factor % factor == 0) {
            answer.push(new_factor);
        }

        nums = nums.iter().filter_map(|&number| { if number % new_factor == 0 {
            Some(number)
        }
        else {
            None
        }}).collect();
    }
    return answer
}