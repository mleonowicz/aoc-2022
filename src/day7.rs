use std::collections::HashMap;

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    let file_sizes = parse(input);

    let mut sum: usize = 0;
    for (_, value) in file_sizes.into_iter() {
        if value <= 100000 {
            sum += value;
        }        
    }
    sum
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> usize {
    let file_sizes = parse(input);

    let used_space: usize = *file_sizes.get(&vec!["/"]).unwrap();
    let free_space: usize = 70000000 - used_space;
    let needed_space: usize = 30000000 - free_space;

    let mut smallest_directory: usize = used_space;
    for (_, value) in file_sizes.into_iter() {
        if value >= needed_space && value < smallest_directory {
            smallest_directory = value;
        }
    }
    smallest_directory
}

fn parse(input: &str) -> HashMap<Vec<&str>, usize> {
    let mut cwd: Vec<&str> = Vec::new();
    let mut file_sizes: HashMap<Vec<&str>, usize> = HashMap::new();

    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let line = line.as_slice();

        match line {
            ["$", "cd", "/"] => {
                cwd.clear();
                cwd.push("/");
            },
            ["$", "cd", ".."] => { cwd.pop(); },
            ["$", "cd", subdir] => { cwd.push(subdir); },
            ["$", "ls"] => (),
            ["dir", _] => (),
            [size, _] => {
                let size = size.parse::<usize>().unwrap();
                for i in 0..cwd.len() {
                    file_sizes
                        .entry(cwd[0..=i].to_vec())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
            }
            _ => unreachable!(),
        }
    }
    file_sizes
}