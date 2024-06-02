static MAX_VECTOR_SIZE: usize = 300;
static MAX_INT_SIZE: u32 = 100000000;

struct Solution {

}

impl Solution {
    pub fn count_triplets_naive(array: &[u32]) -> u32 {
        let mut tuples: Vec<(u32, u32, u32)> = Vec::new();

        let length = array.len() - 1;

        for start in 0..length {
            let mut part1 = 0;

            for mid in start..=length {
                part1 ^= array[mid];
                let mut part2 = part1;

                for end in (mid + 1)..=length {
                    part2 ^= array[end];

                    if part2 == 0 {
                        tuples.push((start as u32, mid as u32, end as u32));
                        println!("({start}, {mid}, {end})");
                    }
                }
            }
        }
        tuples.len() as u32
    }
}

fn main() {
    let vector: Vec<u32> = Vec::from([2,3,1,6,7]);

    assert!(vector.len().le(&MAX_VECTOR_SIZE));
    assert_eq!(vector.iter().find(|&x| { x > &MAX_INT_SIZE}),None);

    println!("The number of triplets is: {}", Solution::count_triplets(&vector));
}