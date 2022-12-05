#[aoc(day5, part1)]
fn solve_part1(input: &str) -> String {
    solve(input, false)
}


#[aoc(day5, part2)]
fn solve_part2(input: &str) -> String {
    solve(input, true)
}

fn solve(input: &str, retain_order: bool) -> String {
    let input_parts: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let crates: Vec<&str> = input_parts[0]
        .lines()
        .rev()
        .skip(1)
        .collect();

    let operations = input_parts[1]
        .lines()
        .map(|op| op
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        )
        .collect::<Vec<Vec<usize>>>();

    // initializaing columns vectors
    let num_of_columns = (crates[0].len() + 1) / 4;
    let mut columns: Vec<Vec<char>> = Vec::with_capacity(num_of_columns);
    for _ in 0..num_of_columns {
        columns.push(Vec::with_capacity(0));
    }
    
    // initializing columns crates
    for line in crates {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                columns[i].push(c);
            }
        }
    }
    
    // moving crates
    for op in operations {
        let count = op[0];
        let from = op[1] - 1;
        let to = op[2] - 1;

        let l = columns[from].len();
        let temp = columns[from].split_off(l - count);

        if retain_order {
            columns[to].extend(temp);
        }
        else {
            columns[to].extend(temp.iter().rev());
        }
    }

    let mut answer = String::from("");
    for col in columns {
        if col.len() > 0 {
            answer.push(col.last().copied().unwrap());
        }
    }
    answer
}
