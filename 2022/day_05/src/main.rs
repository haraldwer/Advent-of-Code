use std::fs; 

fn puzzle_1(input:&str)
{
    println!(" - Puzzle 1 - ");

    let lines = input.lines().collect::<Vec<&str>>();
    let stack_end_index = lines.iter().position(|line| !line.as_bytes().iter().any(|x| *x == '[' as u8)).unwrap();
    let stack_count = lines[stack_end_index].len() / 3;

    // Create and allocate stacks
    let mut stacks : Vec::<Vec::<char>> = Vec::with_capacity(stack_count);
    stacks.resize(stack_count, Vec::new());

    // Parse stacks
    for line in lines.iter().take(stack_end_index) {
        for byte in line.as_bytes().iter().skip(1).step_by(4).enumerate() {
            if (*byte.1 as char).is_alphabetic() {
                stacks[byte.0].insert(0, *byte.1 as char);
            }
        }
    }

    // Parse instructions
    for line in lines.iter().skip(stack_end_index + 2)
    {
        let split = line.split(' ').collect::<Vec<&str>>();
        let repeat = split[1].parse::<i32>().unwrap();
        let origin = split[3].parse::<i32>().unwrap() - 1;
        let target : i32 = split[5].parse::<i32>().unwrap() - 1;
        for _ in 0..repeat {
            let val = stacks[origin as usize].pop().unwrap();
            stacks[target as usize].push(val);
        }
    }

    // Compile result
    let mut result = String::new();
    for stack in stacks
    {
        match stack.last()
        {
            Some(n) => result.push(*n),
            None => continue,
        }
    }
    println!("Result: {result}");
}

fn puzzle_2(input:&str)
{
    println!(" - Puzzle 2 - ");

    let lines = input.lines().collect::<Vec<&str>>();
    let stack_end_index = lines.iter().position(|line| !line.as_bytes().iter().any(|x| *x == '[' as u8)).unwrap();
    let stack_count = lines[stack_end_index].len() / 3;

    // Create and allocate stacks
    let mut stacks : Vec::<Vec::<char>> = Vec::with_capacity(stack_count);
    stacks.resize(stack_count, Vec::new());

    // Parse stacks
    for line in lines.iter().take(stack_end_index) {
        for byte in line.as_bytes().iter().skip(1).step_by(4).enumerate() {
            if (*byte.1 as char).is_alphabetic() {
                stacks[byte.0].insert(0, *byte.1 as char);
            }
        }
    }

    // Parse instructions
    for line in lines.iter().skip(stack_end_index + 2)
    {
        let split = line.split(' ').collect::<Vec<&str>>();
        let repeat = split[1].parse::<i32>().unwrap();
        let origin = split[3].parse::<i32>().unwrap() - 1;
        let target : i32 = split[5].parse::<i32>().unwrap() - 1;

        let offset = stacks[origin as usize].len() as i32 - repeat;
        let val = stacks[origin as usize].split_off(offset as usize);
        stacks[target as usize].extend(val);
    }

    // Compile result
    let mut result = String::new();
    for stack in stacks
    {
        match stack.last()
        {
            Some(n) => result.push(*n),
            None => continue,
        }
    }
    println!("Result: {result}");
}

fn main() {
    
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    puzzle_1(&input);
    puzzle_2(&input);
}
