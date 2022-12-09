use std::collections::HashSet;

fn traverse<'a, I>(vec: &Vec<u8>, set: &mut HashSet<usize>, indices: I) where I: Iterator<Item = usize> {
    let mut max = -1;
    for index in indices {
        let val = vec[index as usize] as i32;
        if val > max {
            set.insert(index);
            max = val; 
        }
    }
}

fn puzzle_1(vec: &Vec<u8>, x_size: usize, y_size: usize) {
    println!(" - Puzzle 1 - ");
    let mut set = HashSet::new();
    for y in 0..y_size {
        let itr = (y_size * y)..(y_size * y + x_size);
        traverse(&vec, &mut set, itr.clone());
        traverse(&vec, &mut set, itr.clone().rev());
    }
    for x in 0..x_size {
        let itr = (x..(x_size * y_size)).step_by(x_size);
        traverse(&vec, &mut set, itr.clone());
        traverse(&vec, &mut set, itr.clone().rev());
    }
    let total = set.len();
    println!("Total: {total}");
}


fn check<'a, I>(vec: &Vec<u8>, base_height: u8, indices: I) -> i32 where I: Iterator<Item = usize> {
    let mut c: i32 = 0;
    for index in indices {
        if vec[index] >= base_height {
            return c + 1;
        }
        c += 1;
    }
    return c;
}

fn puzzle_2(vec: &Vec<u8>, x_size: usize, y_size: usize) {
    println!(" - Puzzle 2 - ");
    let mut max = 0;
    for index in 0..(x_size * y_size) {
        let base_x = index % y_size; 
        let base_y = index / x_size; 
        let base_height = vec[index]; 
        let mut sum: i32 = 1;
        sum = sum * check(&vec, base_height, ((base_y * x_size)..index).rev());
        sum = sum * check(&vec, base_height, (index + 1)..((base_y + 1) * x_size));
        sum = sum * check(&vec, base_height, (base_x..index).step_by(x_size).rev());
        sum = sum * check(&vec, base_height, (index..(x_size * y_size)).skip(x_size).step_by(x_size));
        if sum > max {
            max = sum; 
        }
    }
    println!("Best score: {max}");
}

fn main() {

    let input = std::fs::read_to_string("input.txt").expect("failed to read file");

    let lines = input.lines().collect::<Vec<&str>>();
    let y_size = lines.len();
    let x_size = lines[0].len();
    let mut vec = Vec::new();
    vec.resize(x_size * y_size, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            let index = y * x_size + x; 
            vec[index] = *c as u8 - '0' as u8;
        }
    }

    puzzle_1(&vec, x_size, y_size);
    puzzle_2(&vec, x_size, y_size);
}