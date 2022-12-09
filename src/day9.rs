use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(self: &Self, other: &Position) -> i32 {
        i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }

    fn dx(self: &Self, other: &Position) -> i32 {
        self.x - other.x
    }

    fn dy(self: &Self, other: &Position) -> i32 {
        self.y - other.y
    }

    fn update(self: &mut Self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

struct Rope {
    positions: Vec<Position>,
    seen_tail_positions: HashSet<Position>
}

impl Rope {
    fn update_head(self: &mut Self, x: i32, y: i32) {
        self.positions[0].update(x, y);
        self.update_tail_position();
    }

    fn update_tail_position(self: &mut Self) {
        for i in 0..self.positions.len()-1 {
            let distance = self.positions[i].distance(&self.positions[i + 1]);

            let dx = self.positions[i].dx(&self.positions[i + 1]);
            let dy = self.positions[i].dy(&self.positions[i + 1]);
            if (distance == 4 || distance == 3) ||
                (distance == 2 && (i32::abs(dx) == 2 || i32::abs(dy) == 2)) {
                self.positions[i + 1].update(i32::signum(dx), i32::signum(dy)); 
            }
    
            self.seen_tail_positions.insert(*self.positions.last().unwrap());
        }
    }
}

fn solve(input: &str, rope_size: usize) -> usize {
    let mut rope = Rope {
        positions: vec![Position {x: 0, y: 0}; rope_size],
        seen_tail_positions: HashSet::from([Position { x: 0, y: 0}])
    };

    for line in input.lines() {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        let direction = command[0];
        let count = command[1].parse::<i32>().unwrap();
        let [dx, dy] = match direction {
            "R" => [1, 0],
            "D" => [0, -1],
            "L" => [-1, 0],
            "U" => [0, 1],
            _ => unreachable!(),
        };

        for _ in 0..count {
            rope.update_head(dx, dy);
        }
    }
    rope.seen_tail_positions.len()
}

#[aoc(day9, part1)]
fn solve_part1(input: &str) -> usize {
    solve(input, 2)
}   

#[aoc(day9, part2)]
fn solve_part2(input: &str) -> usize {
    solve(input, 10)
}   
