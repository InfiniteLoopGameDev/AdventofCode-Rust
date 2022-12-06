pub fn solution() {
    let data = include_str!("day6.txt");
    for i in 4..data.len() {
        let mut buffer = data[i - 4..i].as_bytes().to_vec();
        buffer.sort();
        buffer.dedup();
        if buffer.len() == 4 {
            println!("Part 1: {}", i);
            break;
        }
    }
    for i in 14..data.len() {
        let mut buffer = data[i - 14..i].as_bytes().to_vec();
        buffer.sort();
        buffer.dedup();
        if buffer.len() == 14 {
            println!("Part 2: {}", i);
            break;
        }
    }
}
