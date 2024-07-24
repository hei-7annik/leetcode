pub fn daily_temperatures(temperatures: Vec<u32>) -> Vec<u32> {
    let mut results= vec![0u32;temperatures.len()];
    for day in 0..temperatures.len() {
        let temp = temperatures[day];

        for future_day in (day + 1)..temperatures.len() {
            if temperatures[future_day] > temp {
                results[day] = (future_day - day) as u32;
                break;
            }
        }
    }
    results
}

/// Determines for each day the duration in day's until the first with a higher temperature value
///
/// # Example
/// ```rust
/// assert_eq!(daily_temperatures([24,25,26,22,29,23,27,24]), "[1,1,4,2,1,1,0,0]")
/// ```
///
/// # Cases
/// 1. No day with a higher temperature value comes after
/// 2. A day with a higher temperature comes after
///
/// # Method
/// 1. Compare temperature of today to temperature of the day on top of the stack
/// 2. If it is colder, push today onto the stack
/// 3. If it is hotter, pop day from the stack
/// 4. Calculate duration in between day and today
/// 5. Repeat from 1 until today gets pushed to the stack
/// 6. Proceed with the next day
///
pub fn daily_temperatures_optimized(mut temperatures: Vec<u32>) -> Vec<u32> {
    let mut stack = Vec::new();

    for day in 0..temperatures.len() {
        while let Some(&some_day_before) = stack.last() {
            if temperatures[day] > temperatures[some_day_before] {
                temperatures[some_day_before] = (day - some_day_before) as u32;
                stack.pop();
            }
            else {
                break;
            }
        }
        stack.push(day);
    }
    stack.iter().for_each(|&i| temperatures[i] = 0);
    temperatures
}