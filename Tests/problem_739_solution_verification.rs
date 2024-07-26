#[cfg(test)]
mod problem_739 {
    use leetcode::problem_739::daily_temperatures::{
        daily_temperatures,
        daily_temperatures_optimized
    };
    mod solution {
        use super::*;

        #[test]
        fn with_rising_and_falling_temperatures() {
            let temps = vec![73,74,75,71,69,72,76,73];
            assert_eq!(daily_temperatures(temps), vec![1,1,4,2,1,1,0,0])
        }

        #[test]
        fn with_rising_temperatures() {
            let temps = vec![30,60,90];
            assert_eq!(daily_temperatures(temps), vec![1,1,0])
        }

        #[test]
        fn with_falling_temperatures() {
            let temps = vec![60,50,40,30];
            assert_eq!(daily_temperatures(temps), vec![0,0,0,0])
        }

    }

    mod optimized_solution {
        use super::*;

        #[test]
        fn with_rising_and_falling_temperatures() {
            let temps = vec![73,74,75,71,69,72,76,73];
            assert_eq!(daily_temperatures_optimized(temps), vec![1,1,4,2,1,1,0,0])
        }

        #[test]
        fn with_rising_temperatures() {
            let temps = vec![30,60,90];
            assert_eq!(daily_temperatures_optimized(temps), vec![1,1,0])
        }

        #[test]
        fn with_falling_temperatures() {
            let temps = vec![60,50,40,30];
            assert_eq!(daily_temperatures_optimized(temps), vec![0,0,0,0])
        }
    }
}