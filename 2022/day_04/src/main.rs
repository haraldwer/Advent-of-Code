use std::fs;

fn puzzle_1(input:&str)
{
    println!(" - Puzzle 1 - ");
    let mut sum = 0; 
    let lines = input.lines();
    for line in lines
    {
        let pair = line.trim().split(',').collect::<Vec<&str>>();
        let first_range = pair[0].split('-').collect::<Vec<&str>>();
        let second_range = pair[1].split('-').collect::<Vec<&str>>();
        let amin : u8 = first_range[0].parse().unwrap();
        let amax : u8 = first_range[1].parse().unwrap();
        let bmin : u8 = second_range[0].parse().unwrap();
        let bmax : u8 = second_range[1].parse().unwrap();
        let overlap = !((amin < bmin && amax < bmax) || (amin > bmin && amax > bmax));
        sum += overlap as u32; 
    }

    println!("Sum: {sum}");
}

fn puzzle_2(input:&str)
{
    println!(" - Puzzle 2 - ");
    let mut sum = 0; 
    let lines = input.lines();
    for line in lines
    {
        let pair = line.trim().split(',').collect::<Vec<&str>>();
        let first_range = pair[0].split('-').collect::<Vec<&str>>();
        let second_range = pair[1].split('-').collect::<Vec<&str>>();
        let amin : u8 = first_range[0].parse().unwrap();
        let amax : u8 = first_range[1].parse().unwrap();
        let bmin : u8 = second_range[0].parse().unwrap();
        let bmax : u8 = second_range[1].parse().unwrap();
        let overlap = !(amin > bmax || bmin > amax);
        sum += overlap as u32; 
    }

    println!("Sum: {sum}");
}

fn main() {

   let input = fs::read_to_string("input.txt").expect("Failed to read file");
   puzzle_1(&input);
   puzzle_2(&input);
}
