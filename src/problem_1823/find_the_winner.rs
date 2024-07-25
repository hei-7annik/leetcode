use std::collections::VecDeque;
pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut friends_in_the_game = n;
    let mut friends: Vec<i32> = (1..=n).collect();

    let mut index = 0;
    while friends_in_the_game > 1 {

        let mut steps = 0;

        while steps < k {
            if friends[((index + steps) % n) as usize] == 0 {
                index = (index + 1) % n;
            }
            else {
                steps += 1;
            }
        }

        index = (index + steps - 1) % n;

        friends[index as usize] = 0;
        friends_in_the_game -= 1;
    }

    *friends.iter().find(|&&friend| friend != 0).unwrap()
}

/// Simulate a game where after each round one player leaves and the last remaining player wins.
/// Each round consists of counting from zero to a predetermined number `k` in steps of one. In
/// clockwise order the players in the circle call out the next number until one hits `k` and leaves
/// the game.
///
/// # Example
/// ```rust
/// assert_eq!(find_the_winner_optimized(5, 2),3);
/// ```
///
/// # Cases
/// 1. The group of friends consists of only one player
/// 2. The number `k` is smaller than `group.size`
/// 3. The number `k` is larger than `group.size`
///
/// # Method
/// 1. Take position `k % group.size` as partition point
/// 2. Remove the player at that position
/// 3. Simulate the circular arrangement by moving everything after `k` to the front
/// 4. Repeat
///
pub fn find_the_winner_optimized(n: i32, k: i32) -> i32 {
    let mut friends: VecDeque<i32> = (1..=n).collect();

    while friends.len() > 1 {
        let steps = (k - 1) as usize % friends.len();
        friends.rotate_left(steps);

        friends.pop_front();
    }

    friends.pop_front().unwrap()
}