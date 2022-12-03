use std::fs;

fn puzzle_1()
{
    println!(" - Puzzle 1 - ");

    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = input.split('\n');
    let mut total = 0;
    for line in lines
    {
        if line.len() < 3
            { continue; }

        let opponent_pick = line.as_bytes()[0] - 'A' as u8;
        let your_pick = line.as_bytes()[2] - 'X' as u8;
        let outcome = 
            (((your_pick + 2) % 3) == opponent_pick) as i32 * 3 -
            (((your_pick + 1) % 3) == opponent_pick) as i32 * 3 + 3; 
        let shape = your_pick as i32 + 1;
        let round_total = shape + outcome;
        total += round_total;
    }

    println!("Total {total}");
}

fn puzzle_2()
{
    println!(" - Puzzle 2 - ");

    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = input.split('\n');
    let mut total = 0;
    for line in lines
    {
        if line.len() < 3
            { continue; }

        let opponent_pick = line.as_bytes()[0] as i32 - 'A' as i32;
        let your_result = line.as_bytes()[2] as i32 - 'X' as i32;
        let your_pick = (opponent_pick + (2 + your_result)) % 3;
        let outcome = your_result * 3; 
        let shape = your_pick + 1;
        let round_total = shape + outcome;
        total += round_total;
    }

    println!("Total {total}");
}

fn main() {
    puzzle_1();
    puzzle_2();
}
