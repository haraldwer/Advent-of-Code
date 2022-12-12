use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let mut map: Vec<u8> = Vec::new();
    let mut start: i32 = 0; 
    let mut end: i32 = 0;
    let width = input.lines().nth(0).unwrap().len() as i32;
    let height = input.lines().count() as i32;
    for line in input.lines() {
        for byte in line.as_bytes() {
            match *byte as char {
                'S' => { 
                    start = map.len() as i32;
                    map.push(0);
                }
                'E' => { 
                    end = map.len() as i32;
                    map.push('z' as u8 - 'a' as u8);
                }
                _ => { 
                    if *byte >= 'a' as u8 && *byte <= 'z' as u8 {
                        map.push(*byte - 'a' as u8); 
                    }
                }
            }
        }
    }

    let mut depth = 0;
    let mut traversed: HashSet<i32> = HashSet::new();
    let mut step_to: HashSet<i32> = HashSet::new();

    // puzzle 1
    //step_to.insert(start);

    // puzzle 2
    for x in map.iter().enumerate().filter(|x| *x.1 == 0) {
        step_to.insert(x.0 as i32); 
    }

    for _ in 0..height * width {
        let curr_step = step_to.clone();
        step_to.clear(); 
        for index in curr_step {
            if index == end {
                println!("Found path with {depth} steps");
                return; 
            }
            traversed.insert(index);
            let val = map[index as usize] as i32;
            let x = index % width;
            let y = index / width; 
            let mut try_step = |new_index: i32| {
                let diff = map[new_index as usize] as i32 - val;
                if diff < 2 && !traversed.contains(&new_index) {
                    step_to.insert(new_index);
                }
            };
            try_step((x + 1).clamp(0, width - 1) + width * y);
            try_step((x - 1).clamp(0, width - 1) + width * y);
            try_step((y + 1).clamp(0, height - 1) * width + x);
            try_step((y - 1).clamp(0, height - 1) * width + x);
        }

        depth += 1;
        if step_to.len() == 0 {
            break; 
        }
    }

    println!("Did not find a path");
}
