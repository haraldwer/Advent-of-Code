use std::fs;

struct Coord {
    x: i32,
    y: i32
}

struct Shape {
    parts: Vec<Coord>
}

fn parse_rocks() -> Vec<Shape> {
    let input = fs::read_to_string("rocks.txt").expect("failed to read file");
    let mut shapes = Vec::new();
    let mut y_offset = 0;
    let mut curr_shape = Shape { parts: Vec::new() };

    let mut add_shape = |mut shape: Shape| {
        if shape.parts.len() == 0 {
            return; 
        }

        // flip y
        let mut max = 0;
        for part in &shape.parts {
            if part.y > max {
                max = part.y;
            }
        }
        for part in &mut shape.parts {
            part.y = max - part.y; 
        }

        // add shape
        shapes.push(shape);
    };

    for (line_y, line) in input.lines().enumerate() {
        if line.len() == 0 {
            add_shape(curr_shape);
            curr_shape = Shape { parts: Vec::new() };
            y_offset = line_y + 1;
            continue;
        }
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let y = (line_y - y_offset) as i32;
                curr_shape.parts.push(Coord { 
                    x: x as i32, 
                    y: y as i32 
                }); 
            }
        }
    }
    add_shape(curr_shape);
    return shapes;
}

fn parse_inputs() -> Vec<bool> {
    let input = fs::read_to_string("input_example.txt").expect("failed to read file");
    let mut inputs = Vec::with_capacity(input.len());
    for ch in input.chars() {
        inputs.push(ch == '>');
    }
    return inputs;
}

fn main() {

    let rocks = parse_rocks();
    let inputs = parse_inputs();
    let mut grid: Vec<u8> = Vec::new();
    let mut input_index = 0;

    for rock_index in 0..2022 {

        let shape_index = rock_index % rocks.len();
        let shape = &rocks[shape_index];
        let mut coord = Coord { x: 2, y: 3 };

        let collision = |x, y| -> bool {
            for part in &shape.parts {
                
                let x_off = x + part.x;
                let y_off = y + part.y + grid.len() as i32;

                if x_off < 0 || x_off > 6 {
                    return true;
                }

                if y_off < 0 {
                    return true;
                }

                // compare part to grid
                if y_off < grid.len() as i32 {
                    let mask = grid[y_off as usize];
                    if mask & (1 << (x_off as u8 + 1)) != 0 {
                        return true;
                    }
                }
            }
            return false; 
        };

        loop {

            let input = inputs[input_index];
            input_index += 1;
            if input_index >= inputs.len() {
                input_index = 0;
            }

            // try move shape sideways
            let side = if input { 1 } else { -1 };
            if !collision(coord.x + side, coord.y) {
                coord.x += side;
            }

            // try move shape down
            if collision(coord.x, coord.y - 1) {
                // solidify
                let grid_len = grid.len();
                for part in &shape.parts {
                    let x_off = coord.x + part.x;
                    let y_off = coord.y + part.y + grid_len as i32;
                    if x_off < 0 || x_off > 6 {
                        continue;
                    }
                    while y_off >= grid.len() as i32 {
                        grid.push(0);
                    }
                    grid[y_off as usize] = grid[y_off as usize] | (1 << (x_off as u8 + 1));
                }
                break;
            }
            else {
                coord.y -= 1;
            }
        }
    }

    for row in grid.iter().rev() {
        for x in 0..7 {
            if row & (1 << (x as u8 + 1)) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    let tower_height = grid.len();
    println!("Tower height: {tower_height}");
    // tried 3200, too high
}