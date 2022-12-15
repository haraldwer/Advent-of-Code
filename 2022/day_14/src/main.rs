use std::fs;

struct Coord {
    x: i32,
    y: i32
}

struct Line {
    start: Coord,
    end: Coord,
}

fn get_index(x: i32, y: i32, width: i32, height: i32) -> usize {
    return ((x + width / 2) + (y * height)) as usize;
}

fn check_collision(grid: &Vec<bool>, width: i32, height: i32, lowest_y: i32, x: i32, y: i32) -> bool {
    // check grid
    if grid[get_index(x, y, width, height)] {
        return true; 
    }

    // check floor
    if y >= lowest_y + 2 { // puzzle 2 floor
        return true; 
    }

    return false;
}

fn simulate(lines: &Vec<Line>, lowest_y: i32) -> usize {
    let width = 1000 as i32;
    let height = 1000 as i32;
    let mut grid: Vec<bool> = Vec::new();
    grid.resize((width * height) as usize, false);

    // add lines to grid
    for line in lines {
        let x_min = std::cmp::min(line.start.x, line.end.x);
        let x_max = std::cmp::max(line.start.x, line.end.x) + 1;
        let y_min = std::cmp::min(line.start.y, line.end.y);
        let y_max = std::cmp::max(line.start.y, line.end.y) + 1;
        for y in y_min..y_max {
            for x in x_min..x_max {
                grid[get_index(x, y, width, height)] = true;
            }
        }
    }

    let mut sum = 0; 
    loop {
        let mut grain = Coord{ x: 500, y: 0 };
        loop {
            if !check_collision(&grid, width, height, lowest_y, grain.x, grain.y + 1) {
                grain.y += 1;
                //if grain.y > lowest_y { 
                //    return sum; // puzzle 1 condition
                //}
                continue;
            }
            if !check_collision(&grid, width, height, lowest_y, grain.x - 1, grain.y + 1) {
                grain.x -= 1;
                grain.y += 1;
                continue;
            }
            if !check_collision(&grid, width, height, lowest_y, grain.x + 1, grain.y + 1) {
                grain.x += 1;
                grain.y += 1;
                continue;
            }
            break;
        }
        grid[get_index(grain.x, grain.y, width, height)] = true; 
        sum += 1;
        if grain.x == 500 && grain.y == 0 { 
            return sum;
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

