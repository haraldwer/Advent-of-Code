use std::fs;

#[derive(Clone)]  
struct Coord {
    x: i32,
    y: i32
}

#[derive(Clone)]
struct Sensor {
    coord: Coord,
    dist: i32,
}

fn get_distance(first_x: i32, first_y: i32, second_x:i32, second_y: i32) -> i32 {
    return (first_x - second_x).abs() + (first_y - second_y).abs();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Coord> = Vec::new();
    for line in input.lines() {
        let split = line.split(' ');

        let sensor_loc_str = split
            .clone()
            .skip(2)
            .take(2)
            .collect::<Vec<&str>>();
        let sensor_loc = Coord{
            x: sensor_loc_str[0]
                .replace("x=", "")
                .replace(",", "")
                .parse::<i32>()
                .unwrap(),
            y: sensor_loc_str[1]
                .replace("y=", "")
                .replace(":", "")
                .parse::<i32>()
                .unwrap()
        };

        let beacon_loc_str = split
            .clone()
            .skip(8)
            .take(2)
            .collect::<Vec<&str>>();
        let becaon_loc = Coord {
            x: beacon_loc_str[0]
                .replace("x=", "")
                .replace(",", "")
                .parse::<i32>()
                .unwrap(),
            y: beacon_loc_str[1]
                .replace("y=", "")
                .parse::<i32>()
                .unwrap()
        };

        beacons.push(Coord {
            x: becaon_loc.x,
            y: becaon_loc.y
        });

        sensors.push(Sensor {
            coord: Coord {
                x: sensor_loc.x,
                y: sensor_loc.y
            },
            dist: get_distance(sensor_loc.x, sensor_loc.y, becaon_loc.x, becaon_loc.y)
        });
    }

    // puzzle 1
    let mut min_x = 0;
    let mut max_x = 0;
    for sensor in &sensors {
        if sensor.coord.x - sensor.dist < min_x {
            min_x = sensor.coord.x - sensor.dist;
        }
        if sensor.coord.x + sensor.dist > max_x {
            max_x = sensor.coord.x + sensor.dist;
        }
    }

    let mut sum = 0;
    let y = 2000000;
    for x in min_x..max_x {
        let mut valid = false;
        for sensor in &sensors {
            if get_distance(sensor.coord.x, sensor.coord.y, x, y) <= sensor.dist {
                valid = true;
                break;
            }
        }
        if valid {
            for beacon in &beacons {
                if beacon.x == x && beacon.y == y {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            sum += 1;
        }
    }
    println!("Puzzle 1 result: {sum}");

    // puzzle 2
    let size = 4000000;
    for sensor in &sensors {
        let dist = sensor.dist + 1;
        for y_offset in -dist..dist {
            let y = y_offset + sensor.coord.y;
            if y < 0 || y > size {
                continue;
            }
            let test_x = |x| {
                if x >= 0 && x < size {
                    let mut found = false;
                    for other in &sensors {
                        if get_distance(x, y, other.coord.x, other.coord.y) <= other.dist {
                            found = true;
                        }
                    }
                    if !found {
                        let result = x as u128 * 4000000 + y as u128;
                        println!("Puzzle 2 result: {result}");
                        return true;
                    }
                }
                return false;
            };
            let x_dist = sensor.dist - y_offset.abs() + 1;
            let x_left = sensor.coord.x - x_dist;
            let x_right = sensor.coord.x + x_dist;
            if test_x(x_left) || test_x(x_right) {
                return;
            }
        }
    }    
}
