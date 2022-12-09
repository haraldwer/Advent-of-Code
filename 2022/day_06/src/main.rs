use std::fs; 

fn find_message_index(input:&str, marker_size:i32) -> i32
{
    for line in input.lines() {
        let bytes = line.as_bytes();
        for val in bytes.iter().enumerate() {

            let comp = bytes.iter().enumerate().skip(val.0).take(marker_size as usize);
            if comp.clone().any(|x| comp.clone().any(|y| x.0 != y.0 && *x.1 == *y.1)) {
                continue;
            }
            return val.0 as i32 + marker_size; 
        }
    }
    return -1;
}

fn puzzle_1(input:&str) {
    println!(" - Puzzle 1 - ");
    let index = find_message_index(input, 4);
    println!("Index: {index}");
    
}

fn puzzle_2(input:&str) {
    println!(" - Puzzle 2 - ");
    let index = find_message_index(input, 14);
    println!("Index: {index}");
}

fn main() {    
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    puzzle_1(&input);
    puzzle_2(&input);
}
