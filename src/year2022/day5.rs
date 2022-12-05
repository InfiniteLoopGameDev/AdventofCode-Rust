use regex::Regex;

pub fn solution() {
    let raw_data: [&str; 2] = include_str!("day5.txt")
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    let [cargo_str, actions_str] = raw_data;

    let re = Regex::new(r"(\d+) from (\d+) to (\d+)").unwrap();
    let actions = re.captures_iter(actions_str);

    let cargo_width: usize = cargo_str
        .chars()
        .nth(cargo_str.len() - 1)
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap(); // 9
    let cargo_list = cargo_str.chars().collect::<Vec<char>>();
    let cargo_iter = cargo_list[1..(cargo_str.len() - (cargo_width * 4))]
        .iter()
        .step_by(4);
    let mut cargo: Vec<String> = vec![String::new(); cargo_width];
    let mut x = 0;
    for char in cargo_iter {
        if char != &' ' {
            cargo[x] = char.to_string() + &cargo[x];
        }
        x += 1;
        if x >= cargo_width {
            x = 0;
        }
    }

    let mut cargo_1 = cargo.clone();
    let mut cargo_2 = cargo.clone();

    for i in actions {
        let x = i[1].parse::<usize>().unwrap();
        let y = i[2].parse::<usize>().unwrap() - 1;
        let new_y = i[3].parse::<usize>().unwrap() - 1;

        let tmp = cargo_1[y].len() as isize - x as isize;
        let tmp2: usize;
        if tmp < 0 {
            tmp2 = 0;
        } else {
            tmp2 = tmp as usize;
        }
        let extracted = cargo_1[y][tmp2..]
            .to_string()
            .chars()
            .rev()
            .collect::<String>();
        cargo_1[new_y] = cargo_1[new_y].clone() + &extracted;
        cargo_1[y] = cargo_1[y][0..tmp2].to_string();

        let tmp = cargo_2[y].len() as isize - x as isize;
        let tmp2: usize;
        if tmp < 0 {
            tmp2 = 0;
        } else {
            tmp2 = tmp as usize;
        }
        let extracted = cargo_2[y][tmp2..].to_string();
        cargo_2[new_y] = cargo_2[new_y].clone() + &extracted;
        cargo_2[y] = cargo_2[y][0..tmp2].to_string();
    }

    let part_1 = cargo_1
        .iter()
        .map(|x| x.chars().last().unwrap())
        .collect::<String>();
    println!("Part 1: {}", part_1);
    let part_2 = cargo_2
        .iter()
        .map(|x| x.chars().last().unwrap())
        .collect::<String>();
    println!("Part 2: {}", part_2);
}
