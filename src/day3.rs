#[aoc(day3, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut cost: i32 = 0;
    for line in input.lines() {
        let len: usize = line.len();
        let mut items_count = [0; 53];
        let first_compartments = &line[0..(len / 2)];
        let second_compartments = &line[(len / 2)..len];

        for c in first_compartments.chars() {
            let char_priority: i32 = get_char_priority(c);
            items_count[char_priority as usize] = 1;
        }

        for c in second_compartments.chars() {
            let char_priority: i32 = get_char_priority(c);
            if items_count[char_priority as usize] != 0 {
                cost += char_priority;
                break;
            }
        }
    }
    cost
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut items_count = [0; 53];
    let mut priority_sum: i32 = 0;
    let mut i: i32 = 1;
    let max_i: i32 = 3;

    for line in input.lines() {
        for c in line.chars() {
            let char_priority: i32 = get_char_priority(c);
            if items_count[char_priority as usize] == i - 1 {
                items_count[char_priority as usize] = i
            }
            if items_count[char_priority as usize] == max_i {
                priority_sum += char_priority;
                break;
            }
        }
        i = (i % max_i) + 1;
        if i == 1 {
            items_count = [0; 53];
        }
    }
    priority_sum
}

fn get_char_priority(c: char) -> i32 {
    if c as i32 <= 90 {
        (c as i32) - 65 + 27
    }
    else {
        (c as i32) - 96
    }
}
