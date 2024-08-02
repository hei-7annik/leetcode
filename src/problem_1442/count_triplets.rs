pub fn count_triplets(array: &[u32]) -> u32 {
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



/// Searches for every possible two consecutive sequences that are equal in terms of bitwise-xor.
///
/// # Example
/// ```rust
/// assert_eq!(count_triplets(&vec![2,3,1,6,7]), 4);
/// ```
///
/// # Cases
/// 1. Input does contain a sequence for which bitwise-xor results in 0
/// 2. Input doesn't contain such a sequence
///
/// # Method
/// 1. Use a sliding window to examine all continuous sequences of length `n`
/// ```
/// Example: window_size = 3
/// window       |< start i=3 |< end i=6
/// array [1][2] | [3][4][5]  | [6][7][8]
/// ```
/// 2. Check if a sequence `s` results in `0` when reduced with the bitwise-xor operation.
/// 3. If true, include all triplets `(s.start, mid, s.end)` with `mid > start && mid <= end`.
///
/// Because two `1`'s cancel out, `s` must contain either an even amount of numbers with a `1` in
/// a certain position or none. Splitting `s` therefor leaves both sides either with an even amount
/// of numbers with `1` in a certain position resulting in `0` or an uneven amount resulting in `1`
/// for both .
///
///
pub fn count_triplets_optimized(array: &[u32]) -> u32 {
    let mut count = 0;

    // min window size is 2 because it needs to be split
    for window_size in 2..=array.len() {
        let mut xor_result = 0;

        array.iter()
            .take(window_size - 1)
            .for_each(|number| xor_result ^= number);

        let max_offset = array.len() - window_size;
        // move the window
        for offset in 0..=max_offset {

            xor_result ^= array[offset + window_size - 1];
            if xor_result == 0 {
                let start = offset;
                let end = offset + window_size - 1;
                (start + 1..end + 1).for_each(|mid| println!("({start},{mid},{end})"));

                count += (window_size - 1) as u32;
            }
            xor_result ^= array[offset]
        }
    }
    count
}