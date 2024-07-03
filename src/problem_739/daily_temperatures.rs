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