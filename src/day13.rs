#[derive(Debug, PartialEq, Eq)]
enum List {
    Parent(Vec<List>),
    Value(i32)
}

impl List {
    fn parse_line(line: &str) -> List {
        if line.starts_with('[') {
            let line = &line[1..line.len()-1];
            let mut children: Vec<List> = Vec::new();
            if line.is_empty() {
                return List::Parent(children);
            }
            let mut sublist_start = 0;
            let mut depth = 0;
            for (i, c) in line.chars().enumerate() {
                match c {
                    '[' => {
                        depth += 1;
                        if depth == 1 {
                            sublist_start = i;
                        }
                    },
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            children.push(List::parse_line(&line[sublist_start..i+1]))
                        }
                    }
                    ',' => (),
                    _ => {
                        if depth == 0 {
                            children.push(List::parse_line(&line[i..i+1]));
                        }
                    }
                }
            }
            List::Parent(children)
        }
        else {
            List::Value(line.parse::<i32>().unwrap())
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (List::Value(a), List::Value(b)) => a.partial_cmp(b),
            (List::Value(a), List::Parent(b)) => vec![List::Value(*a)].partial_cmp(b),
            (List::Parent(a), List::Parent(b)) => a.partial_cmp(b),
            (List::Parent(a), List::Value(b)) => a.partial_cmp(&vec![List::Value(*b)])
        }
    }
}

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> usize {
    let parsed_input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| List::parse_line(l))
        .collect::<Vec<List>>();

    let mut sum = 0;
    for i in 0..(parsed_input.len() / 2) {
        if parsed_input[i * 2] < parsed_input[i * 2 + 1] {
            sum += i + 1;
        }
    }
    sum
}
