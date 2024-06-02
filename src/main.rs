static MAX_VECTOR_SIZE: usize = 300;
static MAX_INT_SIZE: u32 = 100000000;

struct Solution {

}

impl Solution {
    pub fn count_triplets(array: &[u32]) -> u32 {
        0
    }
}

fn main() {
    let vector: Vec<u32> = Vec::from([2,3,1,6,7]);

    assert!(vector.len().le(&MAX_VECTOR_SIZE));
    assert_eq!(vector.iter().find(|&x| { x > &MAX_INT_SIZE}),None);

    println!("The number of triplets is: {}", Solution::count_triplets(&vector));
}