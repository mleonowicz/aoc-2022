use std::ops::RangeInclusive;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Path {
    max_x: usize,
    min_x: usize,
    max_y: usize,
    min_y: usize,
    nodes: Vec<Node>
}

impl Path {
    fn add_node(self: &mut Self, node: Node) {
        if self.nodes.is_empty() {
            self.max_x = node.x;
            self.min_x = node.x;
            self.max_y = node.y;
            self.min_y = node.y;
        }
        else {
            if self.max_x < node.x {
                self.max_x = node.x;
            }
            if self.min_x > node.x {
                self.min_x = node.x;
            }
            if self.max_y < node.y {
                self.max_y = node.y;
            }
            if self.min_y > node.y {
                self.min_y = node.y;
            }
        }
        self.nodes.push(node);
    }
}

#[derive(Debug)]
struct Board {
    width: usize,
    min_x: usize,
    height: usize,
    min_y: usize,
    board: Vec<Vec<bool>>,
    paths: Vec<Path>,
    iterarions: usize
}

impl Board {
    fn intialize_board(self: &mut Self) {
        self.min_y = 0;
        self.height = self.paths.iter().max_by_key(|p| p.max_y).unwrap().max_y - self.min_y + 1 + 1;
        
        self.min_x = self.paths.iter().min_by_key(|p| p.min_x).unwrap().min_x;
        self.width = self.paths.iter().max_by_key(|p| p.max_x).unwrap().max_x - self.min_x + 1;
        
        if self.height * 3 + 1 > self.width {
            let diff = self.height * 3 + 1 - self.width; 
            self.width = self.height * 3 + 1;
            self.min_x -= diff / 2;
        }
        self.board = vec![vec![false; self.height]; self.width];

        for path in self.paths.iter() {
            for nodes in path.nodes.windows(2) {
                let start_node = &nodes[0];
                let end_node = &nodes[1];

                if start_node.x == end_node.x {
                    if start_node.y < end_node.y {
                        (start_node.y..=end_node.y).for_each(
                            |i| self.board[start_node.x - self.min_x][i - self.min_y] = true
                        )
                    }
                    else {
                        (end_node.y..=start_node.y).for_each(
                            |i| self.board[start_node.x - self.min_x][i - self.min_y] = true
                        )
                    }
                }
                else if start_node.y == end_node.y {
                    if start_node.x < end_node.x {
                        (start_node.x..=end_node.x).for_each(
                            |i| self.board[i - self.min_x][start_node.y - self.min_y] = true
                        )
                    }
                    else {
                        (end_node.x..=start_node.x).for_each(
                            |i| self.board[i - self.min_x][start_node.y - self.min_y] = true
                        )
                    }
                }
                else {
                    panic!("non horizontal/vertial lines");
                }
            }
        }
    }

    fn simulate(self: &mut Self) {
        if 500 < self.min_x || 500 > self.min_x + self.width {
            return;
        }
        
        let mut x = 500 - self.min_x;
        let mut y = 0;

        loop {
            if y == self.height - 1 {
                break;
            }
            if self.board[x][y + 1] == false {
                y += 1;
            }
            else if x == 0 {
                break;
            }
            else if self.board[x - 1][y + 1] == false {
                x -= 1;
                y += 1;
            }
            else if x == self.width - 1 {
                break;
            }
            else if self.board[x + 1][y + 1] == false {
                x += 1;
                y += 1;
            }
            else {
                self.board[x][y] = true;
                self.iterarions += 1;
                x = 500 - self.min_x;
                y = 0;

                if self.board[x][y] == true {
                    break;
                }
            }
        }
    }

    fn simulate_floor(self: &mut Self) {
        if 500 < self.min_x || 500 > self.min_x + self.width {
            return;
        }

        let mut x = 500 - self.min_x;
        let mut y = 0;

        loop {
            if y == self.height - 1 {
                self.board[x][y] = true;
                self.iterarions += 1;
                x = 500 - self.min_x;
                y = 0;
            }
            else if self.board[x][y + 1] == false {
                y += 1;
            }
            else if self.board[x - 1][y + 1] == false {
                x -= 1;
                y += 1;
            }
            else if self.board[x + 1][y + 1] == false {
                x += 1;
                y += 1;
            }
            else {
                self.board[x][y] = true;
                self.iterarions += 1;
                x = 500 - self.min_x;
                y = 0;

                if self.board[x][y] == true {
                    break;
                }
            }
        }
    }
}


#[aoc(day14, part1)]
fn solve_part1(input: &str) -> usize {
    let mut board: Board = Board {
        width: 0,
        min_x: 0,
        height: 0,
        min_y: 0,
        board: Vec::new(),
        paths: Vec::new(),
        iterarions: 0
    };
    for line in input.lines() {
        let line = line.split(" -> ").collect::<Vec<&str>>();
        let mut path: Path = Path {max_x:0, min_x:0, max_y:0, min_y:0, nodes:Vec::new()};

        for pos in line.iter() {
            let mut temp = pos.split(",");
            let x = temp.next().unwrap();
            let x = x.parse::<usize>().unwrap();

            let y = temp.next().unwrap();
            let y = y.parse::<usize>().unwrap();
            path.add_node(Node {x, y})
        }
        board.paths.push(path);
    }
    board.intialize_board();
    board.simulate();

    board.iterarions
}


#[aoc(day14, part2)]
fn solve_part2(input: &str) -> usize {
    let mut board: Board = Board {
        width: 0,
        min_x: 0,
        height: 0,
        min_y: 0,
        board: Vec::new(),
        paths: Vec::new(),
        iterarions: 0
    };
    for line in input.lines() {
        let line = line.split(" -> ").collect::<Vec<&str>>();
        let mut path: Path = Path {max_x:0, min_x:0, max_y:0, min_y:0, nodes:Vec::new()};

        for pos in line.iter() {
            let mut temp = pos.split(",");
            let x = temp.next().unwrap();
            let x = x.parse::<usize>().unwrap();

            let y = temp.next().unwrap();
            let y = y.parse::<usize>().unwrap();
            path.add_node(Node {x, y})
        }
        board.paths.push(path);
    }
    board.intialize_board();
    board.simulate_floor();
    // println!("{:?}", board);

    board.iterarions
}
