
#[derive(PartialEq, Eq, Debug)]
struct Pos(i32, i32);

impl Pos {
    fn distance(self: &Self, other: &Pos) -> usize {
        (i32::abs(self.0 - other.0) + 
            i32::abs(self.1 - other.1)) as usize
    }
}

#[derive(Debug)]
struct Sensor {
    sensor_pos: Pos,
    beacon_pos: Pos,
    radius: usize
}

impl Sensor {
    pub fn new(sensor_pos: Pos, beacon_pos: Pos) -> Self {
        Sensor {
            radius: sensor_pos.distance(&beacon_pos),
            beacon_pos,
            sensor_pos
        }
    }

    fn in_range(self: &Self, pos_to_check: &Pos) -> bool {
        self.sensor_pos.distance(&pos_to_check) <= self.radius
    }
}

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> usize {
    let mut sensors: Vec<Sensor> = Vec::new();
    let y_to_check = 10;

    for line in input.lines() {
        let line = line
            .replace(",", "")
            .replace(":", "")
            .replace("x=", "")
            .replace("y=", "");
        let line = line
            .split_whitespace()
            .map(|s| 
                match s.parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => 0,
                })
            .collect::<Vec<i32>>();

        let sensor = Sensor::new(
            Pos(line[2], line[3]),
            Pos(line[8], line[9])
        );
        sensors.push(sensor);
    }

    let leftmost_sensor = sensors.iter().min_by_key(|s| s.sensor_pos.0 - (s.radius as i32)).unwrap();
    let rightmost_sensor = sensors.iter().max_by_key(|s| s.sensor_pos.0 + (s.radius as i32)).unwrap();

    let leftmost_x = leftmost_sensor.sensor_pos.0 - (leftmost_sensor.radius as i32);
    let rightmost_x = rightmost_sensor.sensor_pos.0 + (rightmost_sensor.radius as i32);

    let mut sum = 0;
    for x in leftmost_x..=rightmost_x {
        let pos_to_check = Pos(x, y_to_check);
        if sensors.iter().any(
            |s| 
            s.in_range(&pos_to_check) && s.sensor_pos != pos_to_check && s.beacon_pos != pos_to_check
        ) {
            sum += 1;
        }
    }
    sum
}


#[aoc(day15, part2)]
fn solve_part2(input: &str) -> i64 {
    let mut sensors: Vec<Sensor> = Vec::new();

    for line in input.lines() {
        let line = line
            .replace(",", "")
            .replace(":", "")
            .replace("x=", "")
            .replace("y=", "");
        let line = line
            .split_whitespace()
            .map(|s| 
                match s.parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => 0,
                })
            .collect::<Vec<i32>>();

        let sensor = Sensor::new(
            Pos(line[2], line[3]),
            Pos(line[8], line[9])
        );
        sensors.push(sensor);
    }

    for y in 0..=4000000 {
        let mut x = 0;
        while x <= 4000000 {
            let mut found = false;
            for sensor in sensors.iter() {
                let pos_to_check = Pos(x, y);
                
                if sensor.in_range(&pos_to_check) {
                    if x < sensor.sensor_pos.0 {
                        let diff = (sensor.sensor_pos.0 - x) + (sensor.radius as i32 - i32::abs(sensor.sensor_pos.1 - y));
                        x += diff + 1;
                    }
                    else {
                        let diff = sensor.radius as i32 - (x - sensor.sensor_pos.0) - i32::abs(sensor.sensor_pos.1 - y);
                        x += diff + 1;
                    }

                    found = true;
                    break;
                }
            }

            if !found {
                return 4000000 * x as i64 + y as i64;
            }
        }
    }
    panic!("Did not find");
}

