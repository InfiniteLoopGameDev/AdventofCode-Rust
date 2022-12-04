pub fn solution() {
    let data = include_str!("day4.txt").split_whitespace();
    let mut count1 = 0;
    let mut count2 = 0;
    for line in data {
        let mut ranges: Vec<Vec<usize>> = vec![];
        for task in line.split(",") {
            let values: Vec<&str> = task.split("-").collect::<Vec<&str>>();
            let numbers: Vec<usize> = values
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            ranges.push((numbers[0]..=numbers[1]).collect())
        }
        ranges.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        let max_range = ranges[1].clone();
        let intersect: Vec<usize> = max_range
            .iter()
            .filter(|&x| ranges[0].contains(x))
            .cloned()
            .collect::<Vec<usize>>();
        if intersect != [] {
            count2 += 1;
        }
        if intersect == ranges[0].clone() {
            count1 += 1;
        }
    }
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}
