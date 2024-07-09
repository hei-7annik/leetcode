use std::collections::VecDeque;

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    if k == 1 {
        return n
    }

    let mut friends: Vec<i32> = (1..=n).collect();
    let mut friends_in_the_game = n;

    let mut index = -1;
    while friends_in_the_game > 1 {
        let mut steps = k;

        while steps > 0 {
            index = (index + 1) % n;
            if friends[index as usize] != 0 {
                steps -= 1;
            }
        }

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