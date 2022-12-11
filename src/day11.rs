use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Monkey
{
    items: VecDeque<i64>,
    left_op: String,
    binop: String,
    right_op: String,
    divisibility_test: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspected_count: usize
}

impl Monkey {
    fn evaluate(self: &Self, val: i64) -> i64 {
        let a = self.left_op.parse::<i64>().unwrap_or(val);
        let b = self.right_op.parse::<i64>().unwrap_or(val);
    
        match self.binop.as_str() {
            "+" => a + b,
            "-" => a - b,
            "/" => a / b,
            "*" => a * b,
            _ => unreachable!(),
        }
    }

    fn test_stress(self: &mut Self, val: i64) -> bool {
        self.inspected_count += 1;
        return val % self.divisibility_test == 0
    }
}

fn solve(input: &str, num_of_rounds: usize, divide_stress: bool) -> usize{
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in input.split("\n\n") {
        let description = monkey
            .lines()
            .collect::<Vec<&str>>();

        let starting_items = description[1]
            .chars()
            .skip(18)
            .collect::<String>();
        let starting_items = starting_items
            .split(", ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<VecDeque<i64>>();

        let operation = description[2]
            .split_whitespace()
            .skip(3)
            .collect::<Vec<&str>>();
        let left_op = operation[0].to_string();
        let binop = operation[1].to_string();
        let right_op = operation[2].to_string();

        let divisibility_test = description[3]
            .split_whitespace()
            .nth(3)
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let true_monkey = description[4]
            .split_whitespace()
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_monkey = description[5]
            .split_whitespace()
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let monkey = Monkey {
            items: starting_items,
            left_op,
            binop,
            right_op,
            divisibility_test,
            true_monkey,
            false_monkey,
            inspected_count: 0
        };
        monkeys.push(monkey);
    }

    let mut common_divisibility_test = 1;
    for monkey in monkeys.iter() {
        common_divisibility_test *= monkey.divisibility_test;
    }

    for _ in 0..num_of_rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let mut new_value = monkeys[i].evaluate(item);
                if divide_stress {
                    new_value /= 3
                }
                new_value %= common_divisibility_test;
                if monkeys[i].test_stress(new_value) {
                    let true_monkey = monkeys[i].true_monkey;
                    monkeys[true_monkey].items.push_back(new_value);
                }
                else {
                    let false_monkey = monkeys[i].false_monkey;
                    monkeys[false_monkey].items.push_back(new_value);
                }
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspected_count);
    monkeys[monkeys.len() - 1].inspected_count * monkeys[monkeys.len() - 2].inspected_count
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    solve(input, 20, true)
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> usize {
    solve(input, 10000, false)
}
