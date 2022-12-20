use std::fs;
use std::collections::HashSet;

struct Coord {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let mut min = Coord { x: 1000, y: 1000, z: 1000 };
    let mut max = Coord { x: -1000, y: -1000, z: -1000 };
    let mut coords = Vec::new(); 
    let padding = 1; 

    for line in input.lines() {
        let split = line.split(",").collect::<Vec<&str>>();
        let coord = Coord {
            x: split[0].parse::<i32>().unwrap(),
            y: split[1].parse::<i32>().unwrap(),
            z: split[2].parse::<i32>().unwrap()
        };
        min.x = std::cmp::min(coord.x - padding, min.x);
        max.x = std::cmp::max(coord.x + padding, max.x);
        min.y = std::cmp::min(coord.y - padding, min.y);
        max.y = std::cmp::max(coord.y + padding, max.y);
        min.z = std::cmp::min(coord.z - padding, min.z);
        max.z = std::cmp::max(coord.z + padding, max.z);
        coords.push(coord);
    }

    let diff_x = (max.x - min.x) + 1;
    let diff_y = (max.y - min.y) + 1;
    let diff_z = (max.z - min.z) + 1;
    println!("Min: {:?} {:?} {:?}", min.x, min.y, min.z);
    println!("Max: {:?} {:?} {:?}", max.x, max.y, max.z);
    println!("Diff: {:?} {:?} {:?}", diff_x, diff_y, diff_z);

    let coord_to_index = |x, y, z| -> usize {
        return 
            ((x - min.x) + 
            (y - min.y) * diff_x + 
            (z - min.z) * diff_x * diff_y) as usize;
    };

    let index_to_coord = |i| -> Coord {

        let x = min.x + (((i as i32) % (diff_y * diff_x)) % diff_x);
        let y = min.y + (((i as i32) % (diff_y * diff_x)) / diff_x);
        let z = min.z + (((i as i32) / diff_y) / diff_y); 
        return Coord { x: x, y: y, z: z };
    };

    let mut arr = Vec::new();
    arr.resize((diff_x * diff_y * diff_z) as usize, false);
    for i in 0..coords.len() {
        let coord = &coords[i];
        let index = coord_to_index(coord.x, coord.y, coord.z);
        arr[index] = true; 
    }

    // Puzzle 1
    let mut sides = 0; 
    for i in 0..coords.len() {
        let coord = &coords[i];

        let check = |x, y, z| {
            if x < min.x || x > max.x || 
                y < min.y || y > max.y ||
                z < min.z || z > max.z {
                return 1; 
            }
            return (!arr[coord_to_index(x, y, z)]) as i32;
        };

        let check_sides = |x, y, z| {
            let mut sides = 0;
            sides += check(x + 1, y, z);
            sides += check(x - 1, y, z);
            sides += check(x, y + 1, z);
            sides += check(x, y - 1, z);
            sides += check(x, y, z + 1);
            sides += check(x, y, z - 1);
            return sides; 
        };

        sides += check_sides(
            coord.x, 
            coord.y, 
            coord.z);
    }
    println!("Puzzle 1 sides: {sides}");

    // Puzzle 2

    let mut sides_2 = 0;
    let mut tested = HashSet::new();
    let mut to_search = HashSet::new();
    let mut next_search = HashSet::new();
    to_search.insert(coord_to_index(min.x, min.y, min.z));
    while to_search.len() > 0 {
        next_search.clear();
        for index in to_search {
            tested.insert(index);

            let mut fill_check = |x, y, z| {
                if x < min.x || x > max.x || 
                    y < min.y || y > max.y ||
                    z < min.z || z > max.z {
                    return; 
                }
                let t_i = coord_to_index(x, y, z);
                if arr[t_i] {
                    sides_2 += 1; 
                } else if !tested.contains(&t_i) {
                    next_search.insert(t_i);
                }
            };

            let coord = index_to_coord(index);
            fill_check(coord.x + 1, coord.y, coord.z); 
            fill_check(coord.x - 1, coord.y, coord.z); 
            fill_check(coord.x, coord.y + 1, coord.z);
            fill_check(coord.x, coord.y - 1, coord.z);
            fill_check(coord.x, coord.y, coord.z + 1);
            fill_check(coord.x, coord.y, coord.z - 1);
        }
        to_search = next_search.clone();
    }
    println!("Puzzle 2 sides: {sides_2}");
}