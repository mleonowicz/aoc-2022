extern crate pathfinding;

use pathfinding::prelude::astar;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize
}

struct Board {
    start: Pos,
    end: Pos,
    width: usize,
    height: usize,
    board: Vec<Vec<usize>>
}

impl Board {
    fn get_neighbours(self: &Self, p: &Pos) -> Vec<(Pos, usize)> {
        let mut neighbours: Vec<(Pos, usize)> = Vec::new();
        // left
        if p.x != 0 {
            if self.board[p.x][p.y] + 1 >= self.board[p.x - 1][p.y] {
                neighbours.push((Pos { x: p.x - 1, y: p.y}, 1));
            }
        }

        // right
        if p.x != self.width - 1 {
            if self.board[p.x][p.y] + 1>= self.board[p.x + 1][p.y] {
                neighbours.push((Pos { x: p.x + 1, y: p.y}, 1));
            }
        }

        // up
        if p.y != 0 {
            if self.board[p.x][p.y] + 1>= self.board[p.x][p.y - 1] {
                neighbours.push((Pos { x: p.x, y: p.y - 1}, 1));
            }
        }

        // down
        if p.y != self.height - 1 {
            if self.board[p.x][p.y] + 1 >= self.board[p.x][p.y + 1] {
                neighbours.push((Pos { x: p.x, y: p.y + 1}, 1));
            }
        }
        neighbours
    }

    fn get_heuristic(self: &Self, p: &Pos) -> usize {
        usize::abs_diff(p.x, self.end.x) + usize::abs_diff(p.y, self.end.y)
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> usize {
    let width = input.lines().collect::<Vec<&str>>()[0].len();
    let height = input.lines().count();

    let mut board = Board {
        start: Pos {x: 0, y: 0},
        end: Pos {x: 0, y: 0},
        width,
        height,
        board: vec![vec![0; height]; width]
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                board.start = Pos { x, y };
                board.board[x][y] = 'a' as usize;
            }
            else if c == 'E' {
                board.end = Pos { x, y };
                board.board[x][y] = 'z' as usize;
            }
            else {
                board.board[x][y] = c as usize;
            }
        }
    }

    let result = astar(
        &board.start, |p| board.get_neighbours(p),
        |p| board.get_heuristic(p),
        |p| *p == board.end
    );
    result.unwrap().1
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> usize {
    let width = input.lines().collect::<Vec<&str>>()[0].len();
    let height = input.lines().count();
    let mut starting_points: Vec<Pos> = Vec::new();

    let mut board = Board {
        start: Pos {x: 0, y: 0},
        end: Pos {x: 0, y: 0},
        width,
        height,
        board: vec![vec![0; height]; width]
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' || c == 'a' {
                starting_points.push(Pos { x, y });
                board.board[x][y] = 'a' as usize;
            }
            else if c == 'E' {
                board.end = Pos { x, y };
                board.board[x][y] = 'z' as usize;
            }
            else {
                board.board[x][y] = c as usize;
            }
        }
    }

    let mut min = width * height;
    for start in starting_points.iter() {
        board.start = *start;
        let result = astar(
            &board.start, |p| board.get_neighbours(p),
            |p| board.get_heuristic(p),
            |p| *p == board.end
        );

        match result {
            Some(r) => {
                let res = r.1;
                if min > res {
                    min = res;
                }
            },
            None => ()
        }
    }
    min
}
