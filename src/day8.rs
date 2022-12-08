#[aoc(day8, part1)]
fn solve_part1(input: &str) -> usize {
    let width = input.lines().count();
    let height = input.lines().next().unwrap().len();
    
    let mut trees: Vec<u32> = vec![0; width * height];
    let mut grid: Vec<u32> = vec![0; 4 * width * height];
    for (x, row) in input.lines().enumerate() {
        for (y, char) in row.chars().enumerate() {
            trees[x * width + y] = char.to_digit(10).unwrap();

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                grid[x * width + y + width * height * 0] = trees[x * width + y];
                grid[x * width + y + width * height * 1] = trees[x * width + y];
                grid[x * width + y + width * height * 2] = trees[x * width + y];
                grid[x * width + y + width * height * 3] = trees[x * width + y];
            }
        }
    }

    for x in 1..width-1 {
        for y in 1..height-1 {
            if grid[x * width + y + width * height * 0] < trees[(x - 1) * width + y] {
                grid[x * width + y + width * height * 0] = trees[(x - 1) * width + y];
            }

            if grid[x * width + y + width * height * 0] < grid[(x - 1) * width + y + width * height * 0] {
                grid[x * width + y + width * height * 0] = grid[(x - 1) * width + y + width * height * 0];
            }

            if grid[x * width + y + width * height * 1] < trees[x * width + (y - 1)] {
                grid[x * width + y + width * height * 1] = trees[x * width + (y - 1)];
            }

            if grid[x * width + y + width * height * 1] < grid[x * width + (y - 1) + width * height * 1] {
                grid[x * width + y + width * height * 1] = grid[x * width + (y - 1) + width * height * 1];
            }
        }
    }

    for x in (1..width-1).rev() {
        for y in (1..height-1).rev() {
            if grid[x * width + y + width * height * 2] < trees[(x + 1) * width + y] {
                grid[x * width + y + width * height * 2] = trees[(x + 1) * width + y];
            }

            if grid[x * width + y + width * height * 2] < grid[(x + 1) * width + y + width * height * 2] {
                grid[x * width + y + width * height * 2] = grid[(x + 1) * width + y + width * height * 2];
            }

            if grid[x * width + y + width * height * 3] < trees[x * width + (y + 1)] {
                grid[x * width + y + width * height * 3] = trees[x * width + (y + 1)];
            }

            if grid[x * width + y + width * height * 3] < grid[x * width + (y + 1) + width * height * 3] {
                grid[x * width + y + width * height * 3] = grid[x * width + (y + 1) + width * height * 3];
            }
        }
    }

    let mut count = width * height;
    for x in 1..width-1 {
        for y in 1..height-1 {
            let a = grid[x * width + y + width * height * 0];
            let b = grid[x * width + y + width * height * 1];
            let c = grid[x * width + y + width * height * 2];
            let d = grid[x * width + y + width * height * 3];

            let mut max = [a, b, c, d];
            max.sort();
            if max[0] >= trees[x * width + y] {
                count -= 1;
            }
        }
    }
    count
}


#[aoc(day8, part2)]
fn solve_part2(input: &str) -> u32 {
    let width = input.lines().count();
    let height = input.lines().next().unwrap().len();
    
    let mut trees: Vec<u32> = vec![0; width * height];
    let mut grid: Vec<u32> = vec![0; 4 * width * height];
    for (x, row) in input.lines().enumerate() {
        for (y, char) in row.chars().enumerate() {
            trees[x * width + y] = char.to_digit(10).unwrap();
        }
    }

    for x in 1..width-1 {
        for y in 1..height-1 {
            let mut count = 0;
            for xx in (0..x).rev() {
                count += 1;

                if trees[x * width + y] <= trees[xx * width + y] {
                    break
                }
            }
            grid[x * width + y + width * height * 0] = count;

            let mut count = 0;
            for yy in (0..y).rev() {
                count += 1;

                if trees[x * width + y] <= trees[x * width + yy] {
                    break;
                }
            }
            grid[x * width + y + width * height * 1] = count;
        }
    }

    for x in (1..width-1).rev() {
        for y in (1..height-1).rev() {
            let mut count = 0;
            for xx in (x+1)..width {
                count += 1;

                if trees[x * width + y] <= trees[xx * width + y] {
                    break;
                }
            }
            grid[x * width + y + width * height * 2] = count;

            let mut count = 0;
            for yy in (y+1)..height {
                count += 1;

                if trees[x * width + y] <= trees[x * width + yy] {
                    break;
                }
            }
            grid[x * width + y + width * height * 3] = count;
        }
    }

    let mut max = 0;
    for x in 1..width-1 {
        for y in 1..height-1 {
            let a = grid[x * width + y + width * height * 0];
            let b = grid[x * width + y + width * height * 1];
            let c = grid[x * width + y + width * height * 2];
            let d = grid[x * width + y + width * height * 3];

            if a * b * c * d > max {
                max = a * b * c * d;
            }
        }
    }
    max
}
