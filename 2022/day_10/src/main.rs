
fn perform_cycle(cycle: &mut u32, sum: &mut i32, x: &i32) {

    // Puzzle 1
    if ((*cycle as i32 + 1) + 20) % 40 == 0 {
        *sum += (*cycle as i32 + 1) * (*x); 
    }
    
    // Puzzle 2
    let c = if (x - ((*cycle as i32) % 40)).abs() < 2 { '#' } else { '.' };
    print!("{c}");
    if *cycle % 40 == 39 {
        print!("\n");
    }

    *cycle += 1;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0; 
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        match split[0] {
            "noop" => { 
                perform_cycle(&mut cycle, &mut sum, &x);
            }
            "addx" => { 
                perform_cycle(&mut cycle, &mut sum, &x);
                perform_cycle(&mut cycle, &mut sum, &x);
                x += split[1].parse::<i32>().unwrap();
            }
            &_ => { }
        }
    }
    println!("Sum: {sum}");
}