#[aoc(day4, part1)]
fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let (lf, rf, ls, rs) = split_values(l);
            if (lf <= ls && rf >= rs) || (ls <= lf && rs >= rf) { 1 } else { 0 }
        }).sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let (lf, rf, ls, rs) = split_values(l);
            if !(rf < ls || rs < lf) { 1 } else { 0 }
        }).sum()
}

fn split_values(line: &str) -> (i32, i32, i32, i32) {
    let splitted: Vec<&str> = line.split(&[',', '-'][..]).collect();
    let parsed: Vec<i32> = splitted
        .iter()
        .map(|p| p.parse::<i32>()
        .unwrap())
        .collect();

    let left_first_range = parsed[0];
    let right_first_range = parsed[1];

    let left_second_range = parsed[2];
    let right_second_range = parsed[3];

    (left_first_range, right_first_range, left_second_range, right_second_range)
}