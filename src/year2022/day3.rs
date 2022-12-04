pub fn solution() {
    let data = include_str!("day3.txt").split_whitespace();
    let data_size = data.clone().count();
    let mut all_characters: Vec<usize> = vec![];
    for line in data.clone() {
        let (section1, section2) = line.split_at(line.len() / 2 as usize);
        let intersect: char = section1
            .chars()
            .filter(|&x| section2.contains(&x.to_string()))
            .next()
            .unwrap();
        if intersect.is_uppercase() {
            all_characters.push(intersect as usize - 38);
        } else {
            all_characters.push(intersect as usize - 96);
        }
    }
    println!("Part 1: {}", all_characters.iter().sum::<usize>());
    let mut bags = data.clone();
    let mut badge_values: Vec<usize> = vec![];
    for _i in (0..data_size).step_by(3) {
        let first_bag = bags.next().unwrap();
        let second_bag = bags.next().unwrap();
        let third_bag = bags.next().unwrap();
        let intersect = first_bag
            .chars()
            .filter(|x| second_bag.contains(&x.to_string()))
            .filter(|x| third_bag.contains(&x.to_string()))
            .next()
            .unwrap();
        if intersect.is_uppercase() {
            badge_values.push(intersect as usize - 38);
        } else {
            badge_values.push(intersect as usize - 96);
        }
    }
    println!("Part 2: {}", badge_values.iter().sum::<usize>())
}
