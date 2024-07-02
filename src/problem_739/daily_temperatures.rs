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

pub fn daily_temperatures_optimized(temperatures: Vec<u32>) -> Vec<u32> {
    temperatures
}