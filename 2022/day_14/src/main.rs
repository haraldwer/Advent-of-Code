use std::fs;

struct Coord {
    x: i32,
    y: i32
}

struct Line {
    start: Coord,
    end: Coord,
}

fn check_collision(sand: &Vec<Coord>, lines: &Vec<Line>, lowest_y: i32, x: i32, y: i32) -> bool {
    for s in sand {
        if s.x == x && s.y == y {
            return true;
        }
    }
    for line in lines {
        let x_min = std::cmp::min(line.start.x, line.end.x);
        let x_max = std::cmp::max(line.start.x, line.end.x);
        if x >= x_min && x <= x_max {
            let y_min = std::cmp::min(line.start.y, line.end.y);
            let y_max = std::cmp::max(line.start.y, line.end.y);
            if y >= y_min && y <= y_max {
                return true;
            }
        }
    }
    if y >= lowest_y + 2 { // puzzle 2 floor
        return true; 
    }
    return false;
}

fn simulate(lines: &Vec<Line>, lowest_y: i32) -> usize {
    let mut sand: Vec<Coord> = Vec::new();
    sand.reserve(30000);
    loop {
        let mut grain = Coord{ x: 500, y: 0 };
        loop {
            if !check_collision(&sand, &lines, lowest_y, grain.x, grain.y + 1) {
                grain.y += 1;
                //if grain.y > lowest_y { 
                //    return sand.len(); // puzzle 1 condition
                //}
                continue;
            }
            if !check_collision(&sand, &lines, lowest_y, grain.x - 1, grain.y + 1) {
                grain.x -= 1;
                grain.y += 1;
                continue;
            }
            if !check_collision(&sand, &lines, lowest_y, grain.x + 1, grain.y + 1) {
                grain.x += 1;
                grain.y += 1;
                continue;
            }
            break;
        }
        if grain.x == 500 && grain.y == 0 { 
            return sand.len() + 1; // puzzle 2 condition
        }
        sand.push(grain);
        if sand.len() % 1000 == 0 {
            let num = sand.len();
            println!("Num: {num}"); 
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let mut lines: Vec<Line> = Vec::new();
    let mut lowest_y = 0;
    for line in input.lines() {
        let coords = line.split(" -> ");
        let mut prev_coord = Coord{ x:-1, y:-1 };
        for coord_str in coords {
            let split = coord_str.split(',').collect::<Vec<&str>>();
            let coord = Coord{
                x: split[0].parse::<i32>().unwrap(),
                y: split[1].parse::<i32>().unwrap()
            };
            if coord.y > lowest_y {
                lowest_y = coord.y;
            }
            if prev_coord.x != -1 && prev_coord.y != -1 {
                lines.push(Line{
                    start: Coord{
                        x: prev_coord.x,
                        y: prev_coord.y
                    },
                    end: Coord {
                        x: coord.x,
                        y: coord.y
                    }
                })
            }
            prev_coord = coord;
        }
    }

    let result = simulate(&lines, lowest_y);
    println!("Result: {result}");

}

