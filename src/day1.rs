#[aoc(day1, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut count = 0;
    let mut max = -1;
    for line in input.lines() {
        if !line.trim().is_empty() {
            count += line.parse::<i32>().unwrap();
        }
        else {
            if count > max {
                max = count
            }
            count = 0;
        }
    }
    if count > max {
        max = count
    }

    max
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut calories: Vec<i32> = Vec::new();
    let mut count = 0;
    for line in input.lines() {
        if !line.trim().is_empty() {
            count += line.parse::<i32>().unwrap();
        }
        else {
            calories.push(count);
            count = 0;
        }
    }
    calories.push(count);
    calories.sort();

    calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3]
}
