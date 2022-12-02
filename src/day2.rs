#[aoc(day2, part1)]
fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|g| 
            match g {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
        
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _     => panic!(),
            }    
        )
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|g| 
            match g {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
        
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                _     => panic!(),
            }    
        )
        .sum()
}
