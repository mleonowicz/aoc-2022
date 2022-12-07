use std::collections::HashMap;

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    let mut cwd: Vec<&str> = Vec::new();
    let commands: Vec<&str> = input.lines().collect();
    let mut file_sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut index: usize = 0;

    loop {
        let command = commands.get(index);
        if command.is_none() {
            break;
        }
        let command = command.unwrap();
        let tokens: Vec<&str> = command.split_whitespace().collect();

        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    cwd.pop();
                }
                else {
                    cwd.push(tokens[2]);
                }
                index += 1;
            }
            else if tokens[1] == "ls" {
                loop {
                    index += 1;

                    let command = commands.get(index);
                    if command.is_none() {
                        break;
                    }
                    let command = command.unwrap();
                    let tokens: Vec<&str> = command.split_whitespace().collect();

                    if tokens[0] == "$" {
                        break;
                    }
                    
                    // Directory case
                    if tokens[0] == "dir" {

                    }
                    // File case
                    else {
                        let file_size: usize = tokens[0].parse::<usize>().unwrap();   
                        let _file_name: &str = tokens[1];
                        for i in 0..cwd.len() {
                            if file_sizes.contains_key(&cwd[0..=i].to_vec()) {
                                *file_sizes.get_mut(&cwd[0..=i].to_vec()).unwrap() += file_size;
                            }
                            else {
                                file_sizes.insert(cwd[0..=i].to_vec(), file_size);
                            }
                        }
                    }
                }
            }
        }
    }
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
    let mut cwd: Vec<&str> = Vec::new();
    let commands: Vec<&str> = input.lines().collect();
    let mut file_sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut index: usize = 0;

    loop {
        let command = commands.get(index);
        if command.is_none() {
            break;
        }
        let command = command.unwrap();
        let tokens: Vec<&str> = command.split_whitespace().collect();

        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    cwd.pop();
                }
                else {
                    cwd.push(tokens[2]);
                }
                index += 1;
            }
            else if tokens[1] == "ls" {
                loop {
                    index += 1;

                    let command = commands.get(index);
                    if command.is_none() {
                        break;
                    }
                    let command = command.unwrap();
                    let tokens: Vec<&str> = command.split_whitespace().collect();

                    if tokens[0] == "$" {
                        break;
                    }
                    
                    // Directory case
                    if tokens[0] == "dir" {

                    }
                    // File case
                    else {
                        let file_size: usize = tokens[0].parse::<usize>().unwrap();   
                        let _file_name: &str = tokens[1];
                        for i in 0..cwd.len() {
                            if file_sizes.contains_key(&cwd[0..=i].to_vec()) {
                                *file_sizes.get_mut(&cwd[0..=i].to_vec()).unwrap() += file_size;
                            }
                            else {
                                file_sizes.insert(cwd[0..=i].to_vec(), file_size);
                            }
                        }
                    }
                }
            }
        }
    }
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
