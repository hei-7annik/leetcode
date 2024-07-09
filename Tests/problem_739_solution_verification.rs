#[cfg(test)]
mod problem_739 {
    use leetcode::problem_739::daily_temperatures::{daily_temperatures, daily_temperatures_optimized};
    mod unoptimized_solution {
        use super::*;

        #[test]
        fn with_() {
            let temps = vec![73,74,75,71,69,72,76,73];
            assert_eq!(daily_temperatures(temps), vec![1,1,4,2,1,1,0,0])
        }

        #[test]
        fn with_increasing_temps_short() {
            let temps = vec![30,60,90];
            assert_eq!(daily_temperatures(temps), vec![1,1,0])
        }

        #[test]
        fn with_increasing_temps_long() {
            let temps = vec![30,40,50,60];
            assert_eq!(daily_temperatures(temps), vec![1,1,1,0])
        }

    }

    mod partially_optimized_solution {
        use super::*;

        #[test]
        fn with_() {
            let temps = vec![73,74,75,71,69,72,76,73];
            assert_eq!(daily_temperatures_optimized(temps), vec![1,1,4,2,1,1,0,0])
        }

        #[test]
        fn with_increasing_temps_short() {
            let temps = vec![30,60,90];
            assert_eq!(daily_temperatures_optimized(temps), vec![1,1,0])
        }

        #[test]
        fn with_increasing_temps_long() {
            let temps = vec![30,40,50,60];
            assert_eq!(daily_temperatures_optimized(temps), vec![1,1,1,0])
        }
    }
}