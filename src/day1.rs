#[aoc(day1, part1)]
fn solve_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|l|
            l.lines().map(|n|
                n.parse::<i32>().unwrap()
            ).sum()
        ).max().unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut res: _ = input
        .split("\n\n")
        .map(|l|
            l.lines().map(|n|
                n.parse::<i32>().unwrap()
            ).sum()
        ).collect::<Vec<i32>>();
    res.sort();
    res.iter().rev().take(3).sum::<i32>()
}
