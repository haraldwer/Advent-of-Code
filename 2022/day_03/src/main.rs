use std::fs;

fn byte_to_priority(byte:u8) -> u8
{
    return 
        if byte > 'Z' as u8 { byte + 1 - 'a' as u8 } 
        else { byte + 27 - 'A' as u8 };
}

fn puzzle_1()
{
    println!(" - Puzzle 1 - ");
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut sum = 0;
    let rucksacks = input.split('\n');
    for rucksack in rucksacks
    {
        let mut biggest_item = 0;
        let bytes = rucksack.as_bytes();
        for first_byte in bytes.iter().take(rucksack.len() / 2)
        {
            if bytes.iter().skip(rucksack.len() / 2).any(|x| *x == *first_byte)
            {
                biggest_item = biggest_item.max(byte_to_priority(*first_byte));
            }
        }
        sum += biggest_item as u32;
    }
    println!("Sum: {sum}");
}

fn puzzle_2()
{
    println!(" - Puzzle 2 - ");
    let input = fs::read_to_string("input.txt").expect("Failed to read file");    
    let rucksacks = input.split('\n').collect::<Vec<&str>>();
    let mut index = 0;
    let mut sum : u32 = 0;
    while index < rucksacks.len()
    {
        let mut highest_prio = 0;
        let group = rucksacks.iter().skip(index).take(3).collect::<Vec<&&str>>();
        for first_byte in group[0].trim().as_bytes()
        {
            if group[1].as_bytes().iter().any(|x| *x == *first_byte) && 
                group[2].as_bytes().iter().any(|x| *x == *first_byte)
            {
                highest_prio = highest_prio.max(byte_to_priority(*first_byte));
            }
        }
        sum += highest_prio as u32;
        index += 3; 
    }
    println!("Sum: {sum}");
}

fn main() {
    puzzle_1();
    puzzle_2();
}
