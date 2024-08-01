use std::cmp::Ordering::{Greater, Equal, Less};

/// Calculate the minimum number of candies to distribute among children based on ratings but
/// everyone has to get at least one.
///
/// # Example:
/// ```rust
/// assert_eq!(candy(vec![]), 0);
/// ```
///
/// # Cases:
/// sequence: `[..., m, ... , n, ...]`
/// 1. m > ... > n
/// 2. m < ... < n
///
/// end of a sequence: `[..., m, n, o, ...]`
///
/// 3. m =/</> n = o
/// 4. m > n < o
/// 5. m < n > o
///
/// # Method:
/// 1. Translate each subsequence `s` of increasing/decreasing numbers into a continuous one
///     starting at `1` and ending at `s.length`.
/// 2. Translate each subsequence of equal numbers to `1`'s excluding the first number.
/// 3. Adjust for
/// 3. Sum values
///

pub fn candy(mut ratings: Vec<i32>) -> i32 {
    ratings.push(*ratings.last().unwrap());

    let mut distributed_candies = 0;
    let mut candies = 0;

    let mut threshold = None;
    let mut rating_previous_child = Equal;

    for i in 0..ratings.len() - 1 {
        distributed_candies += candies + 1;

        threshold.is_some_and(|threshold| candies + 1 >= threshold)
            .then(||{ distributed_candies += 1});

        let rating_next_child = ratings[i+1].cmp(&ratings[i]);
        match (rating_previous_child, rating_next_child) {
            // End Sequence
            (Greater, Greater) => {
                threshold = None;
                candies = 1;
            }
            (_, Equal) => {
                threshold = None;
                candies = 0;
            },
            (Less, Less) => {
                threshold = Some(candies + 1);
                candies = 0;
            },
            // Continue Sequence
            _ => candies += 1,
        };
        rating_previous_child = ratings[i].cmp(&ratings[i+1]);
    }
    distributed_candies
}