#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, window_size: usize) -> usize {
    
    let mut left: usize = 0;
    let mut right: usize = window_size - 1;

    let mut sum = 0;
    let mut seen_count: [i32; 128] = [0; 128];
    
    for i in 0..window_size {
        let c = input.chars().nth(i).unwrap();

        seen_count[c as usize] += 1;
        if seen_count[c as usize] == 1 {
            sum += 1;
        }
    }
    for i in 0..(input.len() - window_size) {
        if sum == window_size {
            return i + window_size;
        }
        let left_c = input.chars().nth(left).unwrap();
        
        seen_count[left_c as usize] -= 1;
        if seen_count[left_c as usize] == 0 {
            sum -= 1;
        }

        left += 1;
        right += 1;

        let right_c = input.chars().nth(right).unwrap();

        seen_count[right_c as usize] += 1;
        if seen_count[right_c as usize] == 1 {
            sum += 1;
        }
    }
    panic!("Incorrect input!");
}
