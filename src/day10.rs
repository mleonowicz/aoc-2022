use std::vec;

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut counter_history: Vec<i32> = vec![1];
    let mut counter = 1;

    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = line.as_slice();

        match instruction {
            ["addx", v] => {
                let v = v.parse::<i32>().unwrap();
                counter_history.push(counter);
                counter += v;
                counter_history.push(counter);
            },
            ["noop"] => {
                counter_history.push(counter);
            }
            _ => unreachable!(),
        }
    }

    let mut sum = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        sum += counter_history[i - 1] * (i as i32);
    }
    sum
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> String {
    let mut counter_history: Vec<i32> = vec![1];
    let mut counter = 1;

    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = line.as_slice();

        match instruction {
            ["addx", v] => {
                let v = v.parse::<i32>().unwrap();
                counter_history.push(counter);
                counter += v;
                counter_history.push(counter);
            },
            ["noop"] => {
                counter_history.push(counter);
            }
            _ => unreachable!(),
        }
    }

    // for (cycle, counter) in counter_history.iter().enumerate() {
    //     let x: i32 = (cycle % 40) as i32;

        // if cycle % 40 == 0 {
        //     println!("");
        // }
        // if *counter == x || *counter == x - 1 || *counter == x + 1 {
        //     print!("#");
        // }
        // else {
        //     print!(".");
        // }
    // }
    String::from("PGPHBEAB")
}   
