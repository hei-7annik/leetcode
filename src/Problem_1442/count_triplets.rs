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

///
/// To-Do Add documentation to each function
///
pub fn count_triplets_optimized(array: &[u32]) -> u32 {
    let mut count = 0;

    // min window size is 2 because it needs to be split in two
    for window_size in 2..=array.len() {
        let mut xor_result = 0;

        // setup for sliding window xor calculation
        //
        // Example: window_size = 3
        // window       |< start i=3 |< end i=5
        // array [1][2] | [3][4]     | [5][6][7][8]

        array.iter()
            .take(window_size - 1)
            .for_each(|number| xor_result ^= number);


        // max offset    #  #  #  #  #  |    window    |
        // array        [1][2][3][4][5] | [6][7]...[n] |
        let max_offset = array.len() - window_size;
        for offset in 0..=max_offset {

            // add new window[end] to this xor calculation
            //
            // window       |< start i=3 |< end i=6
            // array [1][2] | [3][4][5]  | [6][7][8]
            xor_result ^= array[offset + window_size - 1];
            if xor_result == 0 {
                let start = offset;
                let end = offset + window_size - 1;
                (start + 1..end + 1).for_each(|mid| println!("({start},{mid},{end})"));

                count += (window_size - 1) as u32;
            }

            // remove old window[start] for next xor calculation
            //
            // window           |< start i=4 |< end i=6
            // array [1][2][3]  | [4][5]     | [6][7][8]
            xor_result ^= array[offset]
        }
    }
    count
}