fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");

    #[derive(Copy)]
    #[derive(Clone)]
    struct Pos { x: i32, y: i32, }    
    let mut knots = [Pos { x:0, y:0 }; 10];
    let mut last_set = std::collections::HashSet::new();
    let mut second_set = std::collections::HashSet::new();
    let width = 10000;
    let height = 10000;

    for line in input.lines() {
        let split = line.split(' ').collect::<Vec<&str>>(); 
        for _ in 0..split[1].parse::<i32>().unwrap() {
            match split[0] {
                "U" => { knots[0].y += 1; }
                "D" => { knots[0].y -= 1; }
                "R" => { knots[0].x += 1; }
                "L" => { knots[0].x -= 1; }
                &_ => { continue; }
            }
            for k in 1..10 {
                let x_diff = knots[k - 1].x - knots[k].x;
                let y_diff = knots[k - 1].y - knots[k].y;
                if std::cmp::max(x_diff.abs(), y_diff.abs()) > 1 {
                    knots[k].x += x_diff.signum();
                    knots[k].y += y_diff.signum();
                } // This really fucked me up
            }
            second_set.insert(knots[2].x + width / 2 + (knots[2].y + height / 2) * width);
            last_set.insert(knots[9].x + width / 2 + (knots[9].y + height / 2) * width);
        }
    }

    let second_positions = second_set.len(); 
    println!("Puzzle 1 positions: {second_positions}");
    let tail_positions = last_set.len(); 
    println!("Puzzle 2 positions: {tail_positions}");
}
