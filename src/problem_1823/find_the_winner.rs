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

pub fn find_the_winner_optimized(n: i32, k: i32) -> i32 {
    let mut friends: VecDeque<i32> = (1..=n).collect();

    while friends.len() > 1 {
        for _i in 0..k - 1 {
            let friend = friends.pop_front().unwrap();
            friends.push_back(friend);
        }

        friends.pop_front();
    }

    friends.pop_front().unwrap()
}