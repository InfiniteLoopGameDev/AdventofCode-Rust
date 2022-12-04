pub fn solution() {
    let data = include_str!("day1.txt");

    let mut max_values: [usize; 3] = [0, 0, 0];

    for block in data.split("\n\n") {
        let block_numbers: Vec<usize> = block
            .trim()
            .split("\n")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let sum: usize = block_numbers.iter().sum();
        let mut new_array_extra = {
            let mut whole: [usize; 4] = [0; 4];
            let (one, two) = whole.split_at_mut(max_values.len());
            one.copy_from_slice(&max_values);
            two.copy_from_slice(&[sum]);
            whole
        };
        new_array_extra.sort_unstable_by(|a, b| b.cmp(a));
        max_values.copy_from_slice(&new_array_extra[0..3]);
    }

    let sum_max: usize = max_values.iter().sum();

    println!("Part 1: {}", max_values[0]);
    println!("Part 2: {}", sum_max);
}
