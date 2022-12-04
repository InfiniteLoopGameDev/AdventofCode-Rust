pub fn solution() {
    let mut data = include_str!("day2.txt").split_whitespace();

    let mut score1: usize = 0;
    let mut score2: usize = 0;

    let winners = [['C', 'B', 'A'], ['X', 'Z', 'Y']];
    let loser = [['C', 'B', 'A'], ['Y', 'X', 'Z']];

    for _i in (0..data.clone().count()).step_by(2) {
        let first_letter = data.next().unwrap().chars().next().unwrap();
        let second_letter = data.next().unwrap().chars().next().unwrap();
        if first_letter as usize + 23 == second_letter as usize {
            score1 += 3;
        } else if winners[1][winners[0].iter().position(|&x| x == first_letter).unwrap()]
            == second_letter
        {
            score1 += 6;
        }
        score1 += second_letter as usize - 87;

        if second_letter == 'X' {
            score2 +=
                loser[1][loser[0].iter().position(|&x| x == first_letter).unwrap()] as usize - 87;
        } else if second_letter == 'Y' {
            score2 += 3;
            score2 += first_letter as usize - 64
        } else {
            score2 += 6;
            score2 += winners[1][winners[0].iter().position(|&x| x == first_letter).unwrap()]
                as usize
                - 87
        }
    }

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);
}
